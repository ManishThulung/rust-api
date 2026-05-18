use std::sync::Arc;

use axum::Router;

use crate::{AppState, config::cors_layer};

mod api;
mod v1;

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .nest("/api", api::api_routes(app_state.clone()))
    .layer(cors_layer())
    .with_state(app_state)
}
