use std::sync::Arc;

use axum::Router;

use crate::AppState;

// use super::v1::v1_routes;
use crate::route::v1::v1_routes;

pub fn api_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
  Router::new().nest("/v1", v1_routes(app_state))
}
