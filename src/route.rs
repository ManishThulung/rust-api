use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
  AppState,
  handler::{signin_handler, signup_handler},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .route("/api/v1/auth/signin", post(signin_handler))
    .route("/api/v1/auth/signup", post(signup_handler))
    .with_state(app_state)
}
