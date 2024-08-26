use std::sync::Arc;

use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

use self::config::Config;

pub mod config;
pub mod db;
pub mod error;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    config: Config,
    pg: PgPool,
}

pub async fn run_server(config: Config, listener: TcpListener) -> anyhow::Result<()> {
    let pg = PgPoolOptions::new()
        .max_connections(8)
        .connect(&config.postgres_dsn)
        .await?;

    let state = Arc::new(AppState { config, pg });

    let router = Router::new()
        .nest("/api", routes::router())
        .with_state(state);

    axum::serve(listener, router).await?;

    Ok(())
}
