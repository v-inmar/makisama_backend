use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a record in the `board_pid` table.
#[derive(Debug, FromRow, Serialize)]
pub struct BoardPid {
    /// The unique identifier of the board_pid record.
    pub id: u64,

    /// The timestamp when the record was created.
    pub datetime_created: NaiveDateTime,

    /// The value associated with the record.
    pub value: String,
}

impl BoardPid {
    /// Creates a new `BoardPid` record in the database with the given `value`.
    ///
    /// This function inserts a new row into the `board_pid` table, then retrieves
    /// the newly created record, returning it as a `BoardPid` object.
    ///
    /// # Arguments
    ///
    /// * `tx` - A mutable reference to a MySQL transaction, used to execute the query.
    /// * `value` - The value to be inserted into the `board_pid` table.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the newly created `BoardPid` or an error.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<BoardPid, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO board_pid (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            BoardPid,
            r#"
            SELECT id, datetime_created, value
            FROM board_pid
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Retrieves a `BoardPid` by its `id`.
    ///
    /// This function queries the `board_pid` table for a record matching the given `id`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool for executing the query.
    /// * `id` - The `id` of the `BoardPid` record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either an `Option<BoardPid>`, which is `Some(BoardPid)` if the record exists,
    /// or `None` if no record is found, or an error if the query fails.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<BoardPid>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            BoardPid,
            r#"
            SELECT id, datetime_created, value
            FROM board_pid
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Retrieves a `BoardPid` by its `value`.
    ///
    /// This function queries the `board_pid` table for a record matching the given `value`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool for executing the query.
    /// * `value` - The `value` of the `BoardPid` record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either an `Option<BoardPid>`, which is `Some(BoardPid)` if the record exists,
    /// or `None` if no record is found, or an error if the query fails.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<BoardPid>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            BoardPid,
            r#"
            SELECT id, datetime_created, value
            FROM board_pid
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
