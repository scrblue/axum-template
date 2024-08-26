use argon2::PasswordHash;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

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

    pub async fn register<'a>(
        pg: &PgPool,

        email_address: &str,
        display_name: &str,
        password_hash: PasswordHash<'a>,
    ) -> sqlx::Result<Self> {
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
}
