use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// Represents a user's authentication identity.
///
/// This struct stores information related to the user's authentication identity in the system.
/// It includes the identity value, creation timestamp, and an optional TTL (Time-to-Live) field
/// which is set when the authentication identity is not actively used.
#[derive(Debug, FromRow, Serialize)]
pub struct UserAuthIdentity {
    /// The unique identifier for the authentication identity.
    pub id: u64,

    /// The authentication value, typically a username or identifier.
    pub value: String,

    /// The optional time-to-live (TTL) for this authentication identity.
    ///
    /// This field is populated when the authentication identity is not actively used,
    /// and the TTL expires when the identity is no longer valid.
    pub datetime_ttl: Option<NaiveDateTime>,

    /// The timestamp when the authentication identity was created.
    pub datetime_created: NaiveDateTime,
}

/*
CREATE TABLE user_auth_identity (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_ttl DATETIME DEFAULT NULL,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
*/

impl UserAuthIdentity {
    /// Creates a new `UserAuthIdentity` record in the database.
    ///
    /// This function inserts a new record into the `user_auth_identity` table with the given
    /// value and returns the newly created `UserAuthIdentity` struct.
    ///
    /// # Arguments
    /// * `tx` - A mutable reference to a `Transaction` used to execute the query.
    /// * `value` - The authentication value to be stored.
    ///
    /// # Returns
    /// A `Result` with the newly created `UserAuthIdentity` struct on success, or an error if
    /// the operation fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<UserAuthIdentity, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_auth_identity (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            UserAuthIdentity,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM user_auth_identity
            WHERE id = LAST_INSERT_ID()
            "#,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Retrieves a `UserAuthIdentity` by its value.
    ///
    /// This method queries the database for a `UserAuthIdentity` record matching the specified
    /// value.
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `value` - The authentication value to search for.
    ///
    /// # Returns
    /// A `Result` with an `Option<UserAuthIdentity>`. If found, the identity is returned.
    /// If no matching identity is found, `None` is returned.
    pub async fn get_by_value(
        pool: &Pool<MySql>,
        value: &str,
    ) -> Result<Option<UserAuthIdentity>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserAuthIdentity,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM user_auth_identity
            WHERE value = ?
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Retrieves a `UserAuthIdentity` by its unique ID.
    ///
    /// This method queries the database for a `UserAuthIdentity` record with the specified
    /// ID.
    ///
    /// # Arguments
    /// * `pool` - A reference to a connection pool used to execute the query.
    /// * `id` - The unique ID of the authentication identity to fetch.
    ///
    /// # Returns
    /// A `Result` with an `Option<UserAuthIdentity>`. If found, the identity is returned.
    /// If no identity is found with the given ID, `None` is returned.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<UserAuthIdentity>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            UserAuthIdentity,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM user_auth_identity
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Updates the TTL (Time-to-Live) of a specific `UserAuthIdentity`.
    ///
    /// This method updates the `datetime_ttl` field of the authentication identity to the
    /// provided TTL value. After updating, it retrieves the updated identity record.
    ///
    /// # Arguments
    /// * `tx` - A mutable reference to a `Transaction` used to execute the query.
    /// * `ttl` - The new TTL value to set for this identity.
    ///
    /// # Returns
    /// A `Result` with the updated `UserAuthIdentity` struct on success, or an error if
    /// the operation fails.
    pub async fn update_ttl(
        &self,
        tx: &mut Transaction<'_, MySql>,
        ttl: &NaiveDateTime,
    ) -> Result<UserAuthIdentity, sqlx::error::Error> {
        sqlx::query!(
            r#"
        UPDATE user_auth_identity SET datetime_ttl = ? WHERE id = ?
        "#,
            ttl,
            self.id
        )
        .execute(&mut **tx)
        .await?;

        // After the update, fetch the updated row
        let row = sqlx::query_as!(
            UserAuthIdentity,
            r#"
            SELECT * FROM user_auth_identity WHERE id = ?
            "#,
            self.id
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }
}
