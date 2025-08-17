use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a record in the `board_user` table.
///
/// This struct models the relationship between a `board` and a `user` in the database. It tracks
/// whether the user is the owner or admin of the board and includes a timestamp for when the
/// user was added or removed from the board. The `datetime_removed` field is an optional field
/// that represents when the user was removed from the board.
#[derive(Debug, FromRow, Serialize)]
pub struct BoardUser {
    /// The unique identifier of the board-user association record.
    pub id: u64,

    /// The timestamp when the record was created.
    pub datetime_created: NaiveDateTime,

    /// The identifier of the associated `board` record.
    pub board_id: u64,

    /// The identifier of the associated `user` record.
    pub user_id: u64,

    /// A boolean indicating whether the user is the owner of the board.
    pub is_owner: bool,

    /// A boolean indicating whether the user is an admin of the board.
    pub is_admin: bool,

    /// The timestamp when the user was removed from the board, if applicable. This is `None`
    /// if the user is still active on the board.
    pub datetime_removed: Option<NaiveDateTime>,
}

impl BoardUser {
    /// Creates a new `BoardUser` record in the database.
    ///
    /// This function inserts a new row into the `board_user` table with the provided `board_id`,
    /// `user_id`, `is_owner`, and `is_admin` values. The boolean fields `is_owner` and `is_admin`
    /// are converted to `i8` (`0` or `1`) for compatibility with MySQL. The `datetime_removed` field
    /// will default to `NULL`, indicating the user is not removed from the board.
    ///
    /// # Arguments
    ///
    /// * `tx` - A mutable reference to a MySQL transaction used to execute the query.
    /// * `board_id` - The `board_id` to associate with the user.
    /// * `user_id` - The `user_id` to associate with the board.
    /// * `is_owner` - A boolean indicating whether the user is the owner of the board.
    /// * `is_admin` - A boolean indicating whether the user is an admin of the board.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the newly created `BoardUser` or an error.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        board_id: u64,
        user_id: u64,
        is_owner: bool,
        is_admin: bool,
    ) -> Result<BoardUser, sqlx::error::Error> {
        // Insert the new board-user record into the `board_user` table
        sqlx::query!(
            r#"
            INSERT INTO board_user (board_id, user_id, is_owner, is_admin, datetime_removed)
            VALUES (?, ?, ?, ?, NULL)
            "#,
            board_id,
            user_id,
            is_owner as i8, // Convert bool to i8 explicitly for SQL
            is_admin as i8  // Convert bool to i8 explicitly for SQL
        )
        .execute(&mut **tx)
        .await?;

        // Fetch the newly inserted board-user record
        let row = sqlx::query!(
            r#"
            SELECT id, datetime_created, board_id, user_id, is_owner, is_admin, datetime_removed
            FROM board_user
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        // Convert the i8 values for is_owner and is_admin to bool
        Ok(BoardUser {
            id: row.id,
            datetime_created: row.datetime_created,
            board_id: row.board_id,
            user_id: row.user_id,
            is_owner: row.is_owner != 0, // i8 to bool conversion
            is_admin: row.is_admin != 0, // i8 to bool conversion
            datetime_removed: row.datetime_removed, // Already Option<NaiveDateTime>
        })
    }

    /// Retrieves a `BoardUser` record by its `id`.
    ///
    /// This function queries the `board_user` table for a record matching the given `id`.
    /// It also ensures that only active records (those where `datetime_removed` is `NULL`) are returned.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool used to execute the query.
    /// * `id` - The `id` of the `BoardUser` record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either an `Option<BoardUser>`, which is `Some(BoardUser)` if the record exists,
    /// or `None` if no record is found, or an error if the query fails.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<BoardUser>, sqlx::error::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, datetime_created, board_id, user_id, is_owner, is_admin, datetime_removed
            FROM board_user
            WHERE id = ?
            AND datetime_removed IS NULL
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        // If no record is found, return None
        if let Some(row) = row {
            Ok(Some(BoardUser {
                id: row.id,
                datetime_created: row.datetime_created,
                board_id: row.board_id,
                user_id: row.user_id,
                is_owner: row.is_owner != 0, // i8 to bool conversion
                is_admin: row.is_admin != 0, // i8 to bool conversion
                datetime_removed: row.datetime_removed, // Already Option<NaiveDateTime>
            }))
        } else {
            Ok(None)
        }
    }

    /// Retrieves a `BoardUser` record by the `board_id` and `user_id`.
    ///
    /// This function queries the `board_user` table for a record matching the given `board_id` and `user_id`.
    /// It ensures that the user has not been removed (i.e., `datetime_removed` is `NULL`).
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool used to execute the query.
    /// * `board_id` - The `board_id` of the `BoardUser` record to retrieve.
    /// * `user_id` - The `user_id` of the `BoardUser` record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either an `Option<BoardUser>`, which is `Some(BoardUser)` if the record exists,
    /// or `None` if no record is found, or an error if the query fails.
    pub async fn get_by_board_id_and_user_id(
        pool: &Pool<MySql>,
        board_id: u64,
        user_id: u64,
    ) -> Result<Option<BoardUser>, sqlx::error::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, datetime_created, board_id, user_id, is_owner, is_admin, datetime_removed
            FROM board_user
            WHERE board_id = ? AND user_id = ?
            AND datetime_removed IS NULL
            "#,
            board_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        // If no record is found, return None
        if let Some(row) = row {
            Ok(Some(BoardUser {
                id: row.id,
                datetime_created: row.datetime_created,
                board_id: row.board_id,
                user_id: row.user_id,
                is_owner: row.is_owner != 0, // i8 to bool conversion
                is_admin: row.is_admin != 0, // i8 to bool conversion
                datetime_removed: row.datetime_removed, // Already Option<NaiveDateTime>
            }))
        } else {
            Ok(None)
        }
    }

    /// Retrieves all `BoardUser` records for a given `user_id` with pagination support.
    ///
    /// This function queries the `board_user` table for all records matching the given `user_id`,
    /// and returns them as a vector of `BoardUser` records with the given page and per_page parameters.
    /// It ensures that only active records (those where `datetime_removed` is `NULL`) are returned.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to a MySQL connection pool used to execute the query.
    /// * `user_id` - The `user_id` of the `BoardUser` records to retrieve.
    /// * `page` - The page number to retrieve (1-based index).
    /// * `per_page` - The number of records per page.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either a `Vec<BoardUser>` with the associated records for the `user_id`,
    /// or an error if the query fails. If no records are found, an empty vector is returned.
    pub async fn get_by_user_id(
        pool: &Pool<MySql>,
        user_id: u64,
        mut page: u64,
        mut per_page: u64,
    ) -> Result<Vec<BoardUser>, sqlx::error::Error> {
        // Set default values if page or per_page are 0
        if page == 0 {
            page = 1; // Default to page 1
        }
        if per_page == 0 {
            per_page = 10; // Default to 10 items per page
        }

        let offset = (page - 1) * per_page;

        // Query to fetch the paginated results
        let rows = sqlx::query!(
            r#"
        SELECT id, datetime_created, board_id, user_id, is_owner, is_admin, datetime_removed
        FROM board_user
        WHERE user_id = ?
        AND datetime_removed IS NULL
        LIMIT ?
        OFFSET ?
        "#,
            user_id,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        // Map the result to a Vec of BoardUser
        let board_users: Vec<BoardUser> = rows
            .into_iter()
            .map(|row| {
                BoardUser {
                    id: row.id,
                    datetime_created: row.datetime_created,
                    board_id: row.board_id,
                    user_id: row.user_id,
                    is_owner: row.is_owner != 0, // i8 to bool conversion
                    is_admin: row.is_admin != 0, // i8 to bool conversion
                    datetime_removed: row.datetime_removed, // Already Option<NaiveDateTime>
                }
            })
            .collect();

        // Return the paginated results (empty vector if no records are found)
        Ok(board_users)
    }
}
