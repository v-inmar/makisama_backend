use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

#[derive(Serialize, Debug, FromRow)]
pub struct UserPidModel {
    pub id: i64,
    pub value: String,
    pub datetime_created: NaiveDateTime,
}

impl UserPidModel {
    // insert new row into user_pid table
    // returns UserPidModel instance with the newly inserted values
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserPidModel, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_pid (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row: UserPidModel = sqlx::query_as!(
            UserPidModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_pid
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    // get single user_pid row by value
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserPidModel>, sqlx::error::Error> {
        let row: Option<UserPidModel> = sqlx::query_as!(
            UserPidModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_pid
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    // get single user_pid row by id
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: i64,
    ) -> Result<Option<UserPidModel>, sqlx::error::Error> {
        let row: Option<UserPidModel> = sqlx::query_as!(
            UserPidModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_pid
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
