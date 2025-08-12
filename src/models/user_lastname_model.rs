use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a user's last name in the system.
///
/// This struct stores information related to a user's last name, including the value
/// of the last name and the timestamp when it was created.
#[derive(Debug, FromRow, Serialize)]
pub struct UserLastname {
    /// The unique identifier for the user's last name record.
    pub id: u64,

    /// The last name of the user.
    ///
    /// This value is unique to each user and cannot be null.
    pub value: String,

    /// The timestamp when the last name was created.
    ///
    /// This field is automatically populated with the current timestamp when the record is created.
    pub datetime_created: NaiveDateTime,
}

/*
CREATE TABLE user_lastname (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
*/

impl UserLastname {
    /// Creates a new `UserLastname` record in the database.
    ///
    /// This function inserts a new record into the `user_lastname` table with the provided
    /// `value` (the last name) and returns the newly created `UserLastname` struct.
    ///
    /// # Arguments
    /// * `tx` - A mutable reference to a `Transaction` used to execute the query.
    /// * `value` - The last name value to be stored.
    ///
    /// # Returns
    /// A `Result` containing the newly created `UserLastname` struct on success, or an error
    /// if the insertion fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserLastname, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_lastname (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            UserLastname,
            r#"
            SELECT id, value, datetime_created
            FROM user_lastname
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Retrieves a `UserLastname` by its unique ID.
    ///
    /// This method queries the database for a `UserLastname` record with the specified
    /// ID.
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `id` - The unique ID of the user's last name record to retrieve.
    ///
    /// # Returns
    /// A `Result` containing an `Option<UserLastname>`. If a record with the specified ID is
    /// found, it is returned. If no matching record is found, `None` is returned.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<UserLastname>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserLastname,
            r#"
            SELECT id, value, datetime_created
            FROM user_lastname
            WHERE id = ? 
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Retrieves a `UserLastname` by its value.
    ///
    /// This method queries the database for a `UserLastname` record with the specified
    /// value (last name).
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `value` - The last name value to search for.
    ///
    /// # Returns
    /// A `Result` containing an `Option<UserLastname>`. If a record with the specified value is
    /// found, it is returned. If no matching record is found, `None` is returned.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserLastname>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserLastname,
            r#"
            SELECT id, value, datetime_created
            FROM user_lastname
            WHERE value = ? 
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
