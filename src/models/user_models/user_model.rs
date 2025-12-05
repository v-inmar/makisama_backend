use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

#[derive(Serialize, Debug, FromRow)]
pub struct UserModel {
    pub id: i64,
    pub password: String, // hashed - use bcrypt
    pub datetime_created: NaiveDateTime,
    pub firstname_id: i64,
    pub lastname_id: i64,
    pub email_id: i64,
    pub pid_id: i64,
    pub authid_id: i64,
    pub datetime_confirmed: Option<NaiveDateTime>,
    pub datetime_deactivated: Option<NaiveDateTime>,
    pub datetime_deleted: Option<NaiveDateTime>,
}

impl UserModel {
    // Create new row in user table in database
    // returns UserModel struct with the newly inserted row
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        hashed_pw: &str,
        fname_id: i64,
        lname_id: i64,
        email_id: i64,
        pid_id: i64,
        authid_id: i64,
    ) -> Result<UserModel, sqlx::error::Error> {
        sqlx::query!(
            // query! is a compile time check query
            r#"
            INSERT INTO user (password, authid_id, email_id, pid_id, firstname_id, lastname_id)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            hashed_pw,
            authid_id,
            email_id,
            pid_id,
            fname_id,
            lname_id
        )
        .execute(&mut **tx)
        .await?;

        let row: UserModel = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, password, datetime_created, firstname_id, lastname_id, email_id, pid_id, authid_id, datetime_confirmed, datetime_deactivated, datetime_deleted
            FROM user
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    // get single user using the authid id
    pub async fn get_by_authid_id(
        pool: &Pool<MySql>,
        authid_id: i64,
    ) -> Result<Option<UserModel>, sqlx::error::Error> {
        let row: Option<UserModel> = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, password, datetime_created, firstname_id, lastname_id, email_id, pid_id, authid_id, datetime_confirmed, datetime_deactivated, datetime_deleted
            FROM user
            WHERE authid_id = ?
            "#,
            authid_id
        ).fetch_optional(pool).await?;

        Ok(row)
    }

    // get single user using the email id
    pub async fn get_by_email_id(
        pool: &Pool<MySql>,
        email_id: i64,
    ) -> Result<Option<UserModel>, sqlx::error::Error> {
        let row: Option<UserModel> = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, password, datetime_created, firstname_id, lastname_id, email_id, pid_id, authid_id, datetime_confirmed, datetime_deactivated, datetime_deleted
            FROM user
            WHERE email_id = ?
            "#,
            email_id
        ).fetch_optional(pool).await?;

        Ok(row)
    }
}
