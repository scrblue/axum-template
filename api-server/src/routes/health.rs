use std::{fmt::Debug, sync::Arc};

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use sqlx::{Connection, PgPool};

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(health_check))
}

#[derive(Serialize)]
struct HealthCheckOut {
    postgresql: HealthStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum HealthStatus {
    Ok,
    Err,
}

impl<T, E: Debug> From<Result<T, E>> for HealthStatus {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(_) => Self::Ok,
            Err(e) => {
                tracing::error!("Error in health check: {e:?}");
                Self::Err
            }
        }
    }
}

async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthCheckOut> {
    let postgresql = check_pg(&state.pg).await.into();

    Json(HealthCheckOut { postgresql })
}

async fn check_pg(pg: &PgPool) -> anyhow::Result<()> {
    let mut pg = pg.acquire().await?;
    pg.ping().await.map_err(Into::into)
}
