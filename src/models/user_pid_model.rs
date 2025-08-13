use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// A struct representing a user identifier (PID) in the database.
#[derive(Debug, FromRow, Serialize)]
pub struct UserPid {
    /// The unique identifier for the user.
    pub id: u64,
    /// The value of the user identifier (PID).
    pub value: String,
    /// The datetime when the user PID was created.
    pub datetime_created: NaiveDateTime,
}

/*
CREATE TABLE user_pid (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(32) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
*/

impl UserPid {
    /// Creates a new `UserPid` entry in the database with the provided `value`.
    ///
    /// This function inserts a new user PID record into the `user_pid` table and returns the
    /// newly created `UserPid` struct.
    ///
    /// # Arguments
    /// * `tx` - The SQL transaction that will be used for executing the query.
    /// * `value` - The unique value to be associated with the user PID.
    ///
    /// # Returns
    /// * `Result<UserPid, sqlx::error::Error>` - A result containing the created `UserPid` struct,
    ///   or an error if the insert or select operation fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserPid, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_pid (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            UserPid,
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

    /// Fetches a `UserPid` record by its unique `id`.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool to query from.
    /// * `id` - The unique identifier of the `UserPid` to retrieve.
    ///
    /// # Returns
    /// * `Result<Option<UserPid>, sqlx::error::Error>` - A result containing an `Option` of `UserPid`,
    ///   which will be `Some(UserPid)` if found, or `None` if no record is found.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<UserPid>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserPid,
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

    /// Fetches a `UserPid` record by its unique `value`.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool to query from.
    /// * `value` - The unique value of the `UserPid` to retrieve.
    ///
    /// # Returns
    /// * `Result<Option<UserPid>, sqlx::error::Error>` - A result containing an `Option` of `UserPid`,
    ///   which will be `Some(UserPid)` if found, or `None` if no record is found.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserPid>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserPid,
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
}
