use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a record in the `board_description` table.
#[derive(Debug, FromRow, Serialize)]
pub struct BoardDescription {
    /// The unique identifier of the board_description record.
    pub id: u64,

    /// The timestamp when the record was created.
    pub datetime_created: NaiveDateTime,

    /// The value associated with the record.
    pub value: String,
}

impl BoardDescription {
    /// Creates a new `BoardDescription` record in the database with the given `value`.
    ///
    /// This function inserts a new row into the `board_description` table, then retrieves
    /// the newly created record, returning it as a `BoardDescription` object.
    ///
    /// # Arguments
    ///
    /// * `tx` - A mutable reference to a MySQL transaction, used to execute the query.
    /// * `value` - The value to be inserted into the `board_description` table.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the newly created `BoardDescription` or an error.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<BoardDescription, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO board_description (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            BoardDescription,
            r#"
            SELECT id, datetime_created, value
            FROM board_description
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Retrieves a `BoardDescription` by its `id`.
    ///
    /// This function queries the `board_description` table for a record matching the given `id`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool for executing the query.
    /// * `id` - The `id` of the `BoardDescription` record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either an `Option<BoardDescription>`, which is `Some(BoardDescription)` if the record exists,
    /// or `None` if no record is found, or an error if the query fails.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<BoardDescription>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            BoardDescription,
            r#"
            SELECT id, datetime_created, value
            FROM board_description
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Retrieves a `BoardDescription` record by its `value`.
    ///
    /// This function queries the `board_description` table for a record matching the given `value`.
    /// It returns the corresponding `BoardDescription` if found, or `None` if no matching record exists.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool used to execute the query.
    /// * `value` - The `value` of the `BoardDescription` record to retrieve. This is the field in the
    ///   `board_description` table that is queried for a match.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// - `Some(BoardDescription)` if a matching record is found, or
    /// - `None` if no record is found, or
    /// - An error if the query fails.
    ///
    /// The `BoardDescription` struct will contain the `id`, `datetime_created`, and `value` fields from
    /// the table.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<BoardDescription>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            BoardDescription,
            r#"
        SELECT id, datetime_created, value
        FROM board_description
        WHERE value = ?
        "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
