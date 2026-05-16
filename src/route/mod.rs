use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod api;
mod v1;

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .nest("/api", api::api_routes())
    .with_state(app_state)
}
