use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod auth;
mod users;

pub fn v1_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
  Router::new()
    .nest("/auth", auth::auth_routes())
    .nest("/users", users::users_routes(app_state))
}
