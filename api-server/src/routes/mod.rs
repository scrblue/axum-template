use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod account;
pub mod health;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/account", account::router())
        .nest("/health-check", health::router())
}
