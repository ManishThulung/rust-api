use axum::{
  extract::{Request, State},
  middleware::Next,
  response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

use crate::{AppState, errors::api_error::AppError, models::UserModel, schema::Claims};

#[derive(Debug, Clone, PartialEq, Eq, Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
  #[sqlx(rename = "user")]
  User,

  #[sqlx(rename = "admin")]
  Admin,

  #[sqlx(rename = "moderator")]
  Moderator,
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
  pub user_id: Uuid,
  pub email: String,
  pub name: String,
  pub role: UserRole,
}

pub async fn auth_middleware(
  State(state): State<Arc<AppState>>,
  mut req: Request,
  next: Next,
) -> Result<Response, AppError> {
  let auth_header = req
    .headers()
    .get("Authorization")
    .and_then(|header| header.to_str().ok());

  let auth_header = match auth_header {
    Some(header) => header,
    None => return Err(AppError::Unauthorized("Unauthorized".to_string())),
  };

  let token = auth_header
    .strip_prefix("Bearer ")
    .ok_or(AppError::Unauthorized("Unauthorized".to_string()))?;

  let decoded = decode::<Claims<Uuid>>(
    token,
    &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
    &Validation::default(),
  )
  .map_err(|e| AppError::Unauthorized("Unauthorized".to_string()))?;

  let user_id = decoded.claims.sub;

  let user = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE id=$1"#)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
      error!("db fetched failed: {}", e);
      AppError::InternalServerError
    })?;

  let user = match user {
    Some(user) => user,
    None => return Err(AppError::Unauthorized("Unauthorized".to_string())),
  };

  let user_detail = CurrentUser {
    user_id: user.id,
    email: user.email,
    name: user.name,
    role: user.role,
  };

  req.extensions_mut().insert(user_detail);
  Ok(next.run(req).await)
}
