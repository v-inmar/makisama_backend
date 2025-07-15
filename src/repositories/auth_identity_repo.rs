use sqlx::{MySql, Pool, Transaction};

use crate::models::auth_identity_model::AuthIdentity;

impl AuthIdentity {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<AuthIdentity, sqlx::error::Error> {
        // insert row
        sqlx::query!(
            r#"
            INSERT INTO auth_identity (value) 
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        // get last inserted row using last insert id
        let auth_identity = sqlx::query_as!(
            AuthIdentity,
            r#"
            SELECT id, value, datetime_ttl, datetime_created 
            FROM auth_identity 
            WHERE id = LAST_INSERT_ID()
            "#,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(auth_identity)
    }

    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: i64,
    ) -> Result<Option<AuthIdentity>, sqlx::error::Error> {
        let auth_identity = sqlx::query_as!(
            AuthIdentity,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM auth_identity
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(auth_identity)
    }

    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<AuthIdentity>, sqlx::error::Error> {
        let auth_identity = sqlx::query_as!(
            AuthIdentity,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM auth_identity
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(auth_identity)
    }
}
