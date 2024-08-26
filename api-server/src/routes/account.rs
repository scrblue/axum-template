use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, routing::post, Json, Router};
use http::StatusCode;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{db, AppState};

lazy_static! {
    static ref ARGON_2: Argon2<'static> = Argon2::default();
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register))
}

#[derive(Deserialize)]
struct RegistrationIn {
    display_name: String,
    email_address: String,
    password: String,
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(RegistrationIn {
        display_name,
        email_address,
        password,
    }): Json<RegistrationIn>,
) -> Result<(), StatusCode> {
    let pg = &state.pg;

    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let hash = ARGON_2.hash_password(password, &salt).map_err(|e| {
        tracing::error!("Argon2 error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let acct = db::user::UserAccount::register(pg, &email_address, &display_name, hash)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => StatusCode::CONFLICT,

            e => {
                tracing::error!("PostgreSQL error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    let acct_id = acct.id();
    tracing::info!("New account for {email_address} has been created; ID = {acct_id}");

    Ok(())
}
