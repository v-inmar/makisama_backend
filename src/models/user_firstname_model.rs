use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a user's first name in the system.
///
/// This struct stores information related to a user's first name, including its value
/// and the timestamp when it was created.
#[derive(Debug, FromRow, Serialize)]
pub struct UserFirstname {
    /// The unique identifier for the user's first name record.
    pub id: u64,

    /// The first name of the user.
    ///
    /// This value is unique to each user and cannot be null.
    pub value: String,

    /// The timestamp when the first name was created.
    ///
    /// This field is automatically populated with the current timestamp when the record is created.
    pub datetime_created: NaiveDateTime,
}

/*

CREATE TABLE user_firstname (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

*/

impl UserFirstname {
    /// Creates a new `UserFirstname` record in the database.
    ///
    /// This function inserts a new record into the `user_firstname` table with the provided
    /// `value` and returns the newly created `UserFirstname` struct.
    ///
    /// # Arguments
    /// * `tx` - A mutable reference to a `Transaction` used to execute the query.
    /// * `value` - The first name to be stored.
    ///
    /// # Returns
    /// A `Result` containing the newly created `UserFirstname` struct on success, or an error
    /// if the insertion fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserFirstname, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_firstname (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            UserFirstname,
            r#"
            SELECT id, value, datetime_created
            FROM user_firstname
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Retrieves a `UserFirstname` by its unique ID.
    ///
    /// This function queries the database for a `UserFirstname` record with the specified ID.
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `id` - The unique ID of the user's first name record to retrieve.
    ///
    /// # Returns
    /// A `Result` containing an `Option<UserFirstname>`. If a record with the specified ID is
    /// found, it is returned. If no matching record is found, `None` is returned.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<UserFirstname>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserFirstname,
            r#"
            SELECT id, value, datetime_created
            FROM user_firstname
            WHERE id = ? 
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Retrieves a `UserFirstname` by its value.
    ///
    /// This function queries the database for a `UserFirstname` record with the specified
    /// value (first name).
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `value` - The first name value to search for.
    ///
    /// # Returns
    /// A `Result` containing an `Option<UserFirstname>`. If a record with the specified value is
    /// found, it is returned. If no matching record is found, `None` is returned.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserFirstname>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserFirstname,
            r#"
            SELECT id, value, datetime_created
            FROM user_firstname
            WHERE value = ? 
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
