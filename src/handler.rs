use crate::AppState;
use crate::errors::api_error::{ApiMessageResponse, ApiResponse, AppError, AppResult};
use crate::extractors::authorization::AdminOnly;
use crate::helpers::jwt::generate_jwt_token;
use crate::models::UserModel;
use crate::schema::{SigninSchema, SignupSchema, UpdateUserSchema};

use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde_json::json;
use sqlx::Error;
use std::sync::Arc;
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

pub async fn signin_handler(
  State(state): State<Arc<AppState>>,
  Json(body): Json<SigninSchema>,
) -> AppResult<impl IntoResponse> {
  let user = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE email=$1"#)
    .bind(body.email)
    .fetch_optional(&state.db)
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

  let token = generate_jwt_token(user.id, &state.jwt_secret).map_err(|e| {
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
  State(state): State<Arc<AppState>>,
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
  let query = sqlx::query_as::<_, UserModel>(
    r#"INSERT INTO users(id,name,password,email) VALUES  ($1, $2, $3, $4) RETURNING *"#,
  )
  .bind(id)
  .bind(body.name)
  .bind(hashed_password)
  .bind(body.email)
  .fetch_one(&state.db)
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

pub async fn test_handler() -> AppResult<impl IntoResponse> {
  Ok((
    StatusCode::CREATED,
    Json(ApiResponse {
      status: "success",
      message: "user registered successfully".to_string(),
      data: Some(6),
    }),
  ))
}

pub async fn get_users_handler(
  State(state): State<Arc<AppState>>,
  // Extension(user): Extension<CurrentUser>,
  AdminOnly(_user): AdminOnly,
) -> AppResult<impl IntoResponse> {
  let users = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users"#)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
      error!("db fetch failed:{}", e);
      AppError::InternalServerError
    })?;

  Ok((
    StatusCode::OK,
    Json(ApiResponse {
      status: "success",
      message: "Users fetched successfully".to_string(),
      data: Some(users),
    }),
  ))
}

pub async fn ger_user_by_id(
  State(state): State<Arc<AppState>>,
  Path(user_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
  let user = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE id=$1"#)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
      error!("db ger_user_by_id failed: {}", e);
      AppError::InternalServerError
    })?;

  match user {
    Some(user) => Ok((
      StatusCode::OK,
      Json(ApiResponse {
        status: "success",
        message: "user fetched successfully".to_string(),
        data: Some(user),
      }),
    )),
    None => Err(AppError::NotFound("User not foune!".to_string())),
  }
}

pub async fn update_user(
  State(state): State<Arc<AppState>>,
  Path(user_id): Path<Uuid>,
  Json(body): Json<UpdateUserSchema>,
) -> AppResult<impl IntoResponse> {
  let updated_user = sqlx::query_as::<_, UserModel>(
    r#"UPDATE users SET name=COALESCE($1, name), email=COALESCE($2, email) WHERE id=$3 RETURNING *"#
  ).bind(body.name)
  .bind(body.email)
  .bind(user_id)
  .fetch_optional(&state.db)
  .await
  .map_err(|e| {
    error!("db updated failed: {}", e);
    AppError::InternalServerError
  })?;

  match updated_user {
    Some(user) => Ok((
      StatusCode::OK,
      Json(ApiResponse {
        status: "success",
        message: "User updated successfully".to_string(),
        data: Some(user),
      }),
    )),
    None => Err(AppError::NotFound("User not found".to_string())),
  }
}

pub async fn delete_user(
  State(state): State<Arc<AppState>>,
  Path(user_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
  let updated_user = sqlx::query_as::<_, UserModel>(r#"DELETE FROM users WHERE id=$1 RETURNING *"#)
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
      error!("db deletion failed: {}", e);
      AppError::InternalServerError
    })?;

  Ok((
    StatusCode::OK,
    Json(ApiResponse {
      status: "success",
      message: "User deleted successfully".to_string(),
      data: Some(updated_user),
    }),
  ))
}
