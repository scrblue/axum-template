use std::sync::Arc;

use argon2::{
    password_hash::{self, rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use axum::{extract::State, routing::post, Json, Router};
use http::StatusCode;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{
    db::user::UserAccount,
    error::{ApplicationError, HttpError},
    AppState,
};

lazy_static! {
    static ref ARGON_2: Argon2<'static> = Argon2::default();
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/log-in", post(log_in))
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
) -> Result<(), HttpError> {
    let pg = &state.pg;

    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let hash = ARGON_2.hash_password(password, &salt).map_err(|e| {
        tracing::error!("Argon2 error: {e}");
        HttpError::internal_server_error()
    })?;

    let acct = UserAccount::register(pg, &email_address, &display_name, hash)
        .await
        .map_err(|e| match e {
            ApplicationError::Db(sqlx::Error::Database(e)) if e.is_unique_violation() => {
                HttpError::new(
                    StatusCode::CONFLICT,
                    Some("The given email address is alraedy in use."),
                )
            }

            e => {
                tracing::error!("Registration error: {e}");
                HttpError::internal_server_error()
            }
        })?;

    let acct_id = acct.id();
    tracing::info!("New account for {email_address} has been created; ID = {acct_id}");

    Ok(())
}

#[derive(Deserialize)]
struct LoginIn {
    email_address: String,
    password: String,
}

async fn log_in(
    State(state): State<Arc<AppState>>,
    Json(LoginIn {
        email_address,
        password,
    }): Json<LoginIn>,
) -> Result<(), HttpError> {
    let pg = &state.pg;

    let Some(account) = UserAccount::fetch_by_email(pg, &email_address)
        .await
        .map_err(|e| {
            tracing::error!("Login error: {e}");
            HttpError::internal_server_error()
        })?
    else {
        return Err(HttpError::unauthorized());
    };

    let hash = account.password_hash().map_err(|e| {
        tracing::error!("Login error: {e}");
        HttpError::internal_server_error()
    })?;

    match ARGON_2.verify_password(password.as_bytes(), &hash) {
        Ok(()) => Ok(()),
        Err(password_hash::Error::Password) => Err(HttpError::unauthorized()),
        Err(e) => {
            tracing::error!("Error in password verification: {e}");
            Err(HttpError::internal_server_error())
        }
    }
}
