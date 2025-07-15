use crate::models::user_model::User;
use sqlx::{MySql, Pool, Transaction};

impl User {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        username: &str,
        email: &str,
        password: &str, // must be already hashed
        firstname_id: i64,
        lastname_id: i64,
        auth_identity_id: i64,
    ) -> Result<User, Box<dyn std::error::Error>> {
        // hash password

        sqlx::query!(
            r#"
            INSERT INTO user (
                email, 
                password, 
                username,
                datetime_deactivated,
                datetime_deleted,
                auth_identity_id,
                firstname_id,
                lastname_id
            )
            VALUES (
                ?,
                ?,
                ?,
                NULL,
                NULL,
                ?,
                ?,
                ?
            )
            "#,
            email.to_lowercase().to_string(),
            password,
            username.to_lowercase().to_string(),
            auth_identity_id,
            firstname_id,
            lastname_id
        )
        .execute(&mut **tx)
        .await?;

        // get last inserted row using last insert id
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, username, datetime_deactivated, datetime_deleted, datetime_created, auth_identity_id, firstname_id, lastname_id
            FROM user
            WHERE
            id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(
        pool: &Pool<MySql>,
        email: &str,
    ) -> Result<Option<User>, sqlx::error::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, username, datetime_deactivated, datetime_deleted, datetime_created, auth_identity_id, firstname_id, lastname_id
            FROM user
            WHERE
            email = ?
            "#,
            email.to_lowercase().to_string()
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_username(
        pool: &Pool<MySql>,
        username: &str,
    ) -> Result<Option<User>, sqlx::error::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, username, datetime_deactivated, datetime_deleted, datetime_created, auth_identity_id, firstname_id, lastname_id
            FROM user
            WHERE
            username = ?
            "#,
            username.to_lowercase().to_string()
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
