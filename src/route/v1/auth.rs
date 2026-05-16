use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
  AppState,
  handler::{signin_handler, signup_handler},
};

pub fn auth_routes() -> Router<Arc<AppState>> {
  Router::new()
    .route("/signin", post(signin_handler))
    .route("/signup", post(signup_handler))
}
