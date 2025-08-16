use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a board name in the system.
///
/// This struct is used to represent the `board_name` table in the database, containing
/// information about a board's name, including its unique identifier, creation timestamp,
/// and the value of the board name.
///
/// The struct also provides methods to create, retrieve, and query `BoardName` records in the database.
#[derive(Debug, FromRow, Serialize)]
pub struct BoardName {
    /// The unique identifier for the board name.
    pub id: u64,

    /// The datetime when the board name was created.
    pub datetime_created: NaiveDateTime,

    /// The value representing the board name.
    pub value: String,
}

impl BoardName {
    /// Creates a new `BoardName` entry in the database.
    ///
    /// This method inserts a new board name record into the `board_name` table with the
    /// provided value and returns the newly created `BoardName` struct.
    ///
    /// # Arguments
    ///
    /// * `tx` - A mutable reference to a database transaction.
    /// * `value` - The value (name) of the board to insert.
    ///
    /// # Returns
    ///
    /// * `Result<BoardName, sqlx::error::Error>` - A result containing the created `BoardName` struct,
    ///   or an error if the insert or select operation fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<BoardName, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO board_name (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            BoardName,
            r#"
            SELECT id, datetime_created, value
            FROM board_name
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Fetches a `BoardName` record by its unique `value`.
    ///
    /// This method queries the `board_name` table to find a record with the specified value.
    /// If found, it returns the `BoardName` struct, otherwise, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to the database connection pool.
    /// * `value` - The value (name) of the board to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<Option<BoardName>, sqlx::error::Error>` - A result containing either the found `BoardName` (if any),
    ///   or `None` if no matching record is found, or an error if the query fails.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<BoardName>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            BoardName,
            r#"
            SELECT id, datetime_created, value
            FROM board_name
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Fetches a `BoardName` record by its unique `id`.
    ///
    /// This method queries the `board_name` table to find a record with the specified `id`.
    /// If found, it returns the `BoardName` struct, otherwise, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to the database connection pool.
    /// * `id` - The unique identifier of the board name to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<Option<BoardName>, sqlx::error::Error>` - A result containing either the found `BoardName` (if any),
    ///   or `None` if no matching record is found, or an error if the query fails.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<BoardName>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            BoardName,
            r#"
            SELECT id, datetime_created, value
            FROM board_name
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
