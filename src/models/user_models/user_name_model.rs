use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

#[derive(Serialize, Debug, FromRow)]
pub struct UserNameModel {
    pub id: i64,
    pub value: String,
    pub datetime_created: NaiveDateTime,
}

impl UserNameModel {
    // insert new row into user_name table
    // returns UserNameModel instance with the newly inserted values
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserNameModel, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_name (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row: UserNameModel = sqlx::query_as!(
            UserNameModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_name
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    // get single user_name row by value
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserNameModel>, sqlx::error::Error> {
        let row: Option<UserNameModel> = sqlx::query_as!(
            UserNameModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_name
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    // get single user_name row by id
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: i64,
    ) -> Result<Option<UserNameModel>, sqlx::error::Error> {
        let row: Option<UserNameModel> = sqlx::query_as!(
            UserNameModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_name
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
