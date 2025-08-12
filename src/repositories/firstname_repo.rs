use crate::models::user_firstname_model::Firstname;
use sqlx::{MySql, Pool, Transaction};

impl Firstname {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<Firstname, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO firstname (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        // get last inserted row using last insert id
        let firstname = sqlx::query_as!(
            Firstname,
            r#"
            SELECT id, value, datetime_created 
            FROM firstname 
            WHERE id = LAST_INSERT_ID()
            "#,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(firstname)
    }

    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<Firstname>, sqlx::error::Error> {
        let firstname = sqlx::query_as!(
            Firstname,
            r#"
            SELECT id, value, datetime_created
            FROM firstname
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(firstname)
    }
}
