use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::MySql;
use sqlx::Pool;
use sqlx::Transaction;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct RevokedTokenModel {
    pub id: i64,
    pub value: String,
    pub datetime_ttl: NaiveDateTime,
    pub datetime_created: NaiveDateTime,
}

impl RevokedTokenModel {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
        datetime_ttl: &NaiveDateTime,
    ) -> Result<RevokedTokenModel, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO revoked_token(value, datetime_ttl)
            VALUES (?, ?)
            "#,
            value,
            datetime_ttl
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            RevokedTokenModel,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM revoked_token
            WHERE
            id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<RevokedTokenModel>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            RevokedTokenModel,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM revoked_token
            WHERE
            value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
