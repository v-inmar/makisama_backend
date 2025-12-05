use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

#[derive(Serialize, Debug, FromRow)]
pub struct UserAuthidModel {
    pub id: i64,
    pub value: String,
    pub datetime_created: NaiveDateTime,
}

impl UserAuthidModel {
    // insert new row into user_authid table
    // returns UserAuthidModel instance with the brand inserted values
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserAuthidModel, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_authid (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            UserAuthidModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_authid
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    // get single user_authid row that matches value
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserAuthidModel>, sqlx::error::Error> {
        let row: Option<UserAuthidModel> = sqlx::query_as!(
            UserAuthidModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_authid
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    // get single user_authid row that matches id
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: i64,
    ) -> Result<Option<UserAuthidModel>, sqlx::error::Error> {
        let row: Option<UserAuthidModel> = sqlx::query_as!(
            UserAuthidModel,
            r#"
            SELECT id, value, datetime_created
            FROM user_authid
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
