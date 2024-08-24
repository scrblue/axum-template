use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod health;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/health-check", health::router())
}
