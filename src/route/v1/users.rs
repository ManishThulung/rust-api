use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
  AppState,
  handler::{delete_user, ger_user_by_id, get_users_handler, update_user},
};

pub fn users_routes() -> Router<Arc<AppState>> {
  Router::new().route("/", get(get_users_handler)).route(
    "/{user_id}",
    get(ger_user_by_id).patch(update_user).delete(delete_user),
  )
}
