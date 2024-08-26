use std::borrow::Cow;

use axum::response::IntoResponse;
use http::StatusCode;
use thiserror::Error;

pub struct HttpError {
    code: StatusCode,
    message: Cow<'static, str>,
}

impl HttpError {
    pub fn new<T: Into<Cow<'static, str>>>(code: StatusCode, message: Option<T>) -> Self {
        if let Some(message) = message {
            let message = message.into();
            Self { code, message }
        } else {
            let message = code.canonical_reason().unwrap_or("OTHER ERROR").into();
            Self { code, message }
        }
    }

    pub fn internal_server_error() -> Self {
        Self::new::<&'static str>(StatusCode::INTERNAL_SERVER_ERROR, None)
    }

    pub fn unauthorized() -> Self {
        Self::new::<&'static str>(StatusCode::UNAUTHORIZED, None)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        (self.code, self.message).into_response()
    }
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("error in the PostgreSQL database: {0}")]
    Db(#[from] sqlx::Error),

    #[error("error in Argon2 hashing: {0}")]
    Argon2(#[from] argon2::Error),

    #[error("error in the application logic: {0}")]
    Internal(Cow<'static, str>),
}
