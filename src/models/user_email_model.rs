use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a user's email address in the system.
///
/// This struct stores information related to a user's email, including the email value
/// and the timestamp when the email was created in the database.
#[derive(Debug, FromRow, Serialize)]
pub struct UserEmail {
    /// The unique identifier for the user's email record.
    pub id: u64,

    /// The email address of the user.
    ///
    /// This value is unique to each user and cannot be null.
    pub value: String,

    /// The timestamp when the email was created.
    ///
    /// This field is automatically populated with the current timestamp when the record is created.
    pub datetime_created: NaiveDateTime,
}

/*
CREATE TABLE user_email (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(255) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
*/

impl UserEmail {
    /// Creates a new `UserEmail` record in the database.
    ///
    /// This function inserts a new record into the `user_email` table with the provided
    /// `value` (email address) and returns the newly created `UserEmail` struct.
    ///
    /// # Arguments
    /// * `tx` - A mutable reference to a `Transaction` used to execute the query.
    /// * `value` - The email address to be stored.
    ///
    /// # Returns
    /// A `Result` containing the newly created `UserEmail` struct on success, or an error
    /// if the insertion fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserEmail, sqlx::error::Error> {
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
            UserEmail,
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

    /// Retrieves a `UserEmail` by its unique ID.
    ///
    /// This method queries the database for a `UserEmail` record with the specified
    /// ID.
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `id` - The unique ID of the user's email record to retrieve.
    ///
    /// # Returns
    /// A `Result` containing an `Option<UserEmail>`. If a record with the specified ID is
    /// found, it is returned. If no matching record is found, `None` is returned.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<UserEmail>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserEmail,
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

    /// Retrieves a `UserEmail` by its value.
    ///
    /// This method queries the database for a `UserEmail` record with the specified
    /// email value.
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `value` - The email address to search for.
    ///
    /// # Returns
    /// A `Result` containing an `Option<UserEmail>`. If a record with the specified value is
    /// found, it is returned. If no matching record is found, `None` is returned.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserEmail>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserEmail,
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
}
