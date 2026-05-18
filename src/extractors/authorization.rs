use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
  errors::api_error::AppError,
  middleware::auth::{CurrentUser, UserRole},
};

pub struct AdminOnly(pub CurrentUser);

impl<S: Sync> FromRequestParts<S> for AdminOnly {
  type Rejection = AppError;

  /**
   * parts only contain request metadata like headers, method, uri, extensions. But not body
   * _: &S is AppState -> Arc<AppState>
   */

  async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
    let user = parts
      .extensions
      .get::<CurrentUser>()
      .cloned()
      .ok_or(AppError::Forbidden(
        "You are not authorized to perfrom this action".to_string(),
      ))?;

    match user.role {
      UserRole::Admin => Ok(AdminOnly(user)),
      _ => Err(AppError::Forbidden(
        "You are not authorized to perfrom this action".to_string(),
      )),
    }
  }
}
