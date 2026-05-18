use std::sync::Arc;

use axum::{
  Router,
  routing::{get, post},
};

use crate::{
  AppState,
  handler::{signin_handler, signup_handler, test_handler},
};

pub fn auth_routes() -> Router<Arc<AppState>> {
  Router::new()
    .route("/signin", post(signin_handler))
    .route("/signup", post(signup_handler))
    .route("/test", get(test_handler))
}
