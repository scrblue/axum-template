use argon2::PasswordHash;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::ApplicationError;

#[allow(unused)]
pub struct UserAccount {
    id: i32,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    is_archived: bool,

    primary_email: i32,

    display_name: String,
    password_hash: String,
}

impl UserAccount {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn password_hash(&self) -> Result<PasswordHash, ApplicationError> {
        PasswordHash::new(&self.password_hash).map_err(|e| {
            ApplicationError::Internal(
                format!(
                    "password hash in DB for user associated with ID {} is invalid: {}",
                    self.id, e
                )
                .into(),
            )
        })
    }

    pub async fn register<'a>(
        pg: &PgPool,

        email_address: &str,
        display_name: &str,
        password_hash: PasswordHash<'a>,
    ) -> Result<Self, ApplicationError> {
        let now = Utc::now();
        let password_hash = password_hash.to_string();

        let mut tx = pg.begin().await?;

        let acct = sqlx::query!(
            "INSERT INTO user_account (created_at, updated_at, display_name, password_hash) \
                VALUES ($1, $1, $2, $3)
                RETURNING id;",
            now,
            display_name,
            password_hash,
        )
        .fetch_one(&mut *tx)
        .await?;

        let id = acct.id;

        let email = sqlx::query!(
            "INSERT INTO user_email_address (user_id, created_at, updated_at, email_address) \
                VALUES ($1, $2, $2, $3)
                RETURNING id;",
            id,
            now,
            email_address,
        )
        .fetch_one(&mut *tx)
        .await?;

        let primary_email = email.id;

        sqlx::query!("UPDATE user_account SET primary_email = $1", primary_email)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(Self {
            id,
            created_at: now,
            updated_at: now,
            is_archived: false,
            primary_email,
            display_name: display_name.to_owned(),
            password_hash,
        })
    }

    /// Returns the user account associated with a given email address should it exist.
    ///
    /// User accounts and email addresses exist in a one-to-many relationship, and this function
    /// will return the user regardless of which associated email is given.
    pub async fn fetch_by_email(
        pg: &PgPool,
        email_address: &str,
    ) -> Result<Option<Self>, ApplicationError> {
        let mut conn = pg.acquire().await?;

        let Some(account) = sqlx::query!(
            "
                SELECT user_account.* \
                    FROM user_account \
                    JOIN user_email_address ON user_email_address.user_id = user_account.id \
                    WHERE user_email_address.email_address = $1;
            ",
            email_address
        )
        .fetch_optional(&mut *conn)
        .await?
        else {
            return Ok(None);
        };

        let Some(primary_email) = account.primary_email else {
            return Err(ApplicationError::Internal(
                format!("account associated with {email_address} does not have a primary email",)
                    .into(),
            ));
        };

        Ok(Some(UserAccount {
            id: account.id,
            created_at: account.created_at,
            updated_at: account.updated_at,
            is_archived: account.is_archived,
            primary_email,
            display_name: account.display_name,
            password_hash: account.password_hash,
        }))
    }
}
