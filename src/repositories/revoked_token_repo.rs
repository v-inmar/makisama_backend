use crate::models::revoked_token_model::RevokedToken;
use chrono::NaiveDateTime;
use sqlx::{MySql, Transaction};

impl RevokedToken {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        token: &str,
        datetime_ttl: NaiveDateTime,
    ) -> Result<RevokedToken, Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO revoked_token (value, datetime_ttl)
            VALUES (?,?)
            "#,
            token,
            datetime_ttl
        )
        .execute(&mut **tx)
        .await?;

        let revoked = sqlx::query_as!(
            RevokedToken,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM revoked_token
            WHERE
            id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(revoked)
    }
}
