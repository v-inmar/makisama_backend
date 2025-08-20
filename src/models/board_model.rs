use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a record in the `board` table.
#[derive(Debug, FromRow, Serialize)]
pub struct Board {
    /// The unique identifier of the board record.
    pub id: u64,

    /// The timestamp when the record was created.
    pub datetime_created: NaiveDateTime,

    /// The timestamp when the record was deleted, if applicable.
    /// This field is optional.
    pub datetime_deleted: Option<NaiveDateTime>,

    /// The identifier of the associated `board_pid` record.
    pub pid_id: u64,

    /// The identifier of the associated `board_name` record.
    pub name_id: u64,

    /// The identifier of the associated `board_description` record, if applicable.
    /// This field is optional.
    pub description_id: Option<u64>,
}

impl Board {
    /// Creates a new `Board` record in the database.
    ///
    /// This function inserts a new row into the `board` table with the provided `pid_id` and `name_id`.
    /// The `description_id` is optional and can be passed as `None` if not available.
    ///
    /// # Arguments
    ///
    /// * `tx` - A mutable reference to a MySQL transaction used to execute the query.
    /// * `pid_id` - The `pid_id` to associate with the new board.
    /// * `name_id` - The `name_id` to associate with the new board.
    /// * `description_id` - An optional `description_id` to associate with the new board.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the newly created `Board` or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the insert operation or fetching of the new record fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        pid_id: u64,
        name_id: u64,
        description_id: Option<u64>,
    ) -> Result<Board, sqlx::error::Error> {
        // Insert the new board record into the `board` table
        sqlx::query!(
            r#"
            INSERT INTO board (pid_id, name_id, description_id)
            VALUES (?, ?, ?)
            "#,
            pid_id,
            name_id,
            description_id
        )
        .execute(&mut **tx)
        .await?;

        // Fetch the newly inserted board record
        let row = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, datetime_deleted, pid_id, name_id, description_id
            FROM board
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Retrieves a `Board` record by its `id`.
    ///
    /// This function queries the `board` table for a record matching the given `id`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool used to execute the query.
    /// * `id` - The `id` of the `Board` record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either an `Option<Board>`, which is `Some(Board)` if the record exists,
    /// or `None` if no record is found, or an error if the query fails.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<Board>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, datetime_deleted, pid_id, name_id, description_id
            FROM board
            WHERE id = ? AND datetime_deleted IS NULL
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Retrieves a `Board` record by its `pid_id`.
    ///
    /// This function queries the `board` table for a record matching the given `pid_id`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool used to execute the query.
    /// * `pid_id` - The `pid_id` of the `Board` record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either an `Option<Board>`, which is `Some(Board)` if the record exists,
    /// or `None` if no record is found, or an error if the query fails.
    pub async fn get_by_pid_id(
        pool: &Pool<MySql>,
        pid_id: u64,
    ) -> Result<Option<Board>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, datetime_deleted, pid_id, name_id, description_id
            FROM board
            WHERE pid_id = ? AND datetime_deleted IS NULL
            "#,
            pid_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn update_datetime_deleted(
        &self,
        tx: &mut Transaction<'_, MySql>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            r#"
            UPDATE board
            SET datetime_deleted = ?
            WHERE id = ?
            "#,
            Utc::now(),
            self.id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
