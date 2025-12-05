use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

#[derive(Serialize, Debug, FromRow)]
pub struct UserEmailModel {
    pub id: i64,
    pub value: String,
    pub datetime_created: NaiveDateTime,
}

impl UserEmailModel {
    // insert new row into user_email table
    // returns UserEmailModel instance with the newly inserted values
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserEmailModel, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_email (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            UserEmailModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_email
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    // get single user_email row by value
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserEmailModel>, sqlx::error::Error> {
        let row: Option<UserEmailModel> = sqlx::query_as!(
            UserEmailModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_email
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    // get single user_email row by id
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: i64,
    ) -> Result<Option<UserEmailModel>, sqlx::error::Error> {
        let row: Option<UserEmailModel> = sqlx::query_as!(
            UserEmailModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_email
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
