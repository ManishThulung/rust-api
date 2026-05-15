use std::sync::Arc;

use axum::{
  Router,
  routing::{get, post},
};

use crate::{
  AppState,
  handler::{
    delete_user, ger_user_by_id, get_users_handler, signin_handler, signup_handler, update_user,
  },
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .route("/api/v1/auth/signin", post(signin_handler))
    .route("/api/v1/auth/signup", post(signup_handler))
    .route("/api/v1/users", get(get_users_handler))
    .route(
      "/api/v1/users/{user_id}",
      get(ger_user_by_id).patch(update_user).delete(delete_user),
    )
    .with_state(app_state)
}
