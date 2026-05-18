use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::middleware::auth::auth_middleware;
use crate::{
  AppState,
  handler::{delete_user, ger_user_by_id, get_users_handler, update_user},
  // middleware::auth_middleware,
};
pub fn users_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
  Router::new()
    .route("/", get(get_users_handler))
    .layer(middleware::from_fn_with_state(
      app_state.clone(),
      auth_middleware,
    ))
    .route(
      "/{user_id}",
      get(ger_user_by_id).patch(update_user).delete(delete_user),
    )
}
