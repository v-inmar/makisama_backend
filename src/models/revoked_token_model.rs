use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{MySql, Pool, Transaction, prelude::FromRow};

/// Represents a revoked token entry in the database.
///
/// This struct is used to represent the `revoked_token` table in the database,
/// containing information about the revoked token, including its ID, value,
/// and associated timestamps for creation and time-to-live (TTL).
///
/// The struct also provides methods for creating a new revoked token and
/// retrieving a revoked token by its value.
#[derive(Debug, Serialize, FromRow)]
pub struct RevokedToken {
    /// The unique identifier for the revoked token.
    pub id: u64,

    /// The value of the revoked token.
    pub value: String,

    /// The time-to-live (TTL) for the revoked token.
    pub datetime_ttl: NaiveDateTime,

    /// The creation timestamp for the revoked token.
    pub datetime_created: NaiveDateTime,
}

impl RevokedToken {
    /// Creates a new revoked token and inserts it into the database.
    ///
    /// This method inserts a new revoked token entry into the `revoked_token` table
    /// with the provided value and TTL, and then fetches the newly created token
    /// from the database, returning it as a `RevokedToken` object.
    ///
    /// # Arguments
    ///
    /// * `tx` - A mutable reference to a database transaction.
    /// * `value` - The value of the revoked token to insert.
    /// * `datetime_ttl` - The TTL for the revoked token.
    ///
    /// # Returns
    ///
    /// * `Result<RevokedToken, sqlx::error::Error>` - A result containing either the
    ///   newly created `RevokedToken` or an error if the operation fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
        datetime_ttl: &NaiveDateTime,
    ) -> Result<RevokedToken, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO revoked_token (value, datetime_ttl)
            VALUES (?, ?)
            "#,
            value,
            datetime_ttl
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            RevokedToken,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM revoked_token
            WHERE
            id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(row)
    }

    /// Retrieves a revoked token by its value from the database.
    ///
    /// This method fetches a single `RevokedToken` from the database using the
    /// provided token value. It returns an `Option` containing the `RevokedToken`
    /// if found, or `None` if no matching token is found.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to the database connection pool.
    /// * `value` - The value of the revoked token to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<Option<RevokedToken>, sqlx::error::Error>` - A result containing either
    ///   an `Option<RevokedToken>` (the found token or None) or an error if the operation fails.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<RevokedToken>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            RevokedToken,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM revoked_token
            WHERE
            value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
