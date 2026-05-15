use crate::AppState;
use crate::errors::api_error::{ApiMessageResponse, ApiResponse, AppError, AppResult};
use crate::helpers::jwt::generate_jwt_token;
use crate::models::UserModel;
use crate::schema::{SigninSchema, SignupSchema};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde_json::json;
use sqlx::Error;
use std::sync::Arc;
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

pub async fn signin_handler(
  State(conn): State<Arc<AppState>>,
  Json(body): Json<SigninSchema>,
) -> AppResult<impl IntoResponse> {
  let user = sqlx::query_as!(
    UserModel,
    r#"SELECT * FROM users WHERE email=$1"#,
    body.email
  )
  .fetch_optional(&conn.db)
  .await
  .map_err(|e| {
    error!("Database error during signin: {:?}", e);
    AppError::InternalServerError
  })?;

  let user = match user {
    Some(user) => user,
    None => {
      return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }
  };

  let valid_password = task::spawn_blocking(move || verify(body.password, &user.password))
    .await
    .map_err(|e| {
      error!("password verification thread failed: {:?}", e);
      AppError::InternalServerError
    })?
    .map_err(|e| {
      error!("Password verification failed: {:?}", e);
      AppError::InternalServerError
    })?;

  if !valid_password {
    return Err(AppError::Unauthorized("Invalid credentials".to_string()));
  }

  let token = generate_jwt_token(user.id).map_err(|e| {
    error!("jwt generation failed: {:?}", e);
    AppError::InternalServerError
  })?;

  info!("user registered successfully");

  Ok((
    StatusCode::OK,
    Json(ApiResponse {
      status: "sucess",
      message: "user login successful".to_string(),
      data: Some(json!({"token": token})),
    }),
  ))
}

pub async fn signup_handler(
  State(conn): State<Arc<AppState>>,
  Json(body): Json<SignupSchema>,
) -> AppResult<impl IntoResponse> {
  // hashing is cpu heavy task. it performs this action in seperate thread and wait for the result
  let hashed_password = task::spawn_blocking(move || hash(body.password, DEFAULT_COST))
    .await
    .map_err(|e| {
      error!("hash password bycrpt theard error: {}", e);
      AppError::InternalServerError
    })?
    .map_err(|e| {
      error!("hash password error: {}", e);
      AppError::InternalServerError
    })?;

  let id = Uuid::new_v4();
  let query = sqlx::query_as!(
    UserModel,
    r#"INSERT INTO users(id,name,password,email) VALUES  ($1, $2, $3, $4) RETURNING *"#,
    &id,
    &body.name,
    hashed_password,
    &body.email
  )
  .fetch_one(&conn.db)
  .await;

  match query {
    Ok(user) => {
      info!("user created successfully: {}", user.email);
      Ok((
        StatusCode::CREATED,
        Json(ApiMessageResponse {
          status: "success",
          message: "user registered successfully".to_string(),
        }),
      ))
    }

    Err(e) => {
      error!("User registration failed: {:?}", e);
      if let Error::Database(db_err) = e {
        error!("got db error: {}", db_err);
        return Err(AppError::Conflict(
          "User already exist. please login instead.".to_string(),
        ));
      }
      Err(AppError::InternalServerError)
    }
  }
}
