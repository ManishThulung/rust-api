use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub status: &'static str,
  pub message: String,
  pub data: Option<T>,
}

#[derive(Serialize)]
pub struct ApiMessageResponse {
  pub status: &'static str,
  pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
  pub status: &'static str,
  pub message: String,
}

#[derive(Debug)]
pub enum AppError {
  Unauthorized(String),
  Conflict(String),
  InternalServerError,
  NotFound(String),
  Forbidden(String),
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    match self {
      AppError::Unauthorized(message) => (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
          status: "error",
          message,
        }),
      )
        .into_response(),

      AppError::Conflict(message) => (
        StatusCode::CONFLICT,
        Json(ErrorResponse {
          status: "error",
          message,
        }),
      )
        .into_response(),

      AppError::NotFound(message) => (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
          status: "error",
          message,
        }),
      )
        .into_response(),

      AppError::Forbidden(message) => (
        StatusCode::FORBIDDEN,
        Json(ErrorResponse {
          status: "error",
          message,
        }),
      )
        .into_response(),

      AppError::InternalServerError => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
          status: "error",
          message: "Something went wrong".to_string(),
        }),
      )
        .into_response(),
    }
  }
}
