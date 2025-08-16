use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool, Transaction};

/// A struct representing a user in the system.
#[derive(Debug, FromRow, Serialize)]
pub struct User {
    /// The unique identifier for the user.
    pub id: u64,
    /// The datetime when the user was created.
    pub datetime_created: NaiveDateTime,
    /// The user's hashed password.
    pub password: String,
    /// The datetime when the user was confirmed (if applicable).
    pub datetime_confirmed: Option<NaiveDateTime>,
    /// The datetime when the user was deactivated (if applicable).
    pub datetime_deactivated: Option<NaiveDateTime>,
    /// The datetime when the user was deleted (if applicable).
    pub datetime_deleted: Option<NaiveDateTime>,
    /// The unique identifier for the user's authentication identity.
    pub auth_identity_id: u64,
    /// The unique identifier for the user's email.
    pub email_id: u64,
    /// The unique identifier for the user's PID (personal identifier).
    pub pid_id: u64,
    /// The unique identifier for the user's first name.
    pub firstname_id: u64,
    /// The unique identifier for the user's last name.
    pub lastname_id: u64,
}

/*

CREATE TABLE `user` (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    password VARCHAR(255) NOT NULL,
    datetime_confirmed DATETIME DEFAULT NULL,
    datetime_deactivated DATETIME DEFAULT NULL,
    datetime_deleted DATETIME DEFAULT NULL,
    auth_identity_id BIGINT UNSIGNED NOT NULL UNIQUE,
    email_id BIGINT UNSIGNED NOT NULL UNIQUE,
    pid_id BIGINT UNSIGNED NOT NULL UNIQUE,
    firstname_id BIGINT UNSIGNED NOT NULL,
    lastname_id BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (auth_identity_id) REFERENCES user_auth_identity(id),
    FOREIGN KEY (email_id) REFERENCES user_email(id),
    FOREIGN KEY (pid_id) REFERENCES user_pid(id),
    FOREIGN KEY (firstname_id) REFERENCES user_firstname(id),
    FOREIGN KEY (lastname_id) REFERENCES user_lastname(id)
);

*/

impl User {
    /// Creates a new `User` entry in the database.
    ///
    /// This function inserts a new user record with the provided information, including the hashed
    /// password, and returns the newly created `User` struct with all fields populated.
    ///
    /// # Arguments
    /// * `tx` - The SQL transaction to be used for executing the insert query.
    /// * `hashed_password` - The hashed password for the user.
    /// * `auth_identity_id` - The unique ID of the user's authentication identity.
    /// * `email_id` - The unique ID of the user's email.
    /// * `pid_id` - The unique ID of the user's pid.
    /// * `firstname_id` - The unique ID of the user's first name.
    /// * `lastname_id` - The unique ID of the user's last name.
    ///
    /// # Returns
    /// * `Result<User, sqlx::error::Error>` - A result containing the created `User` struct, or an error
    ///   if the insert or select operation fails.
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        hashed_password: &str,
        auth_identity_id: u64,
        email_id: u64,
        pid_id: u64,
        firstname_id: u64,
        lastname_id: u64,
    ) -> Result<User, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user (password, auth_identity_id, email_id, pid_id, firstname_id, lastname_id)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            hashed_password,
            auth_identity_id,
            email_id,
            pid_id,
            firstname_id,
            lastname_id
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, datetime_created, password, datetime_confirmed, datetime_deactivated, datetime_deleted, auth_identity_id, email_id, pid_id, firstname_id, lastname_id
            FROM user
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    /// Fetches a `User` record by its unique `id`.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool to query from.
    /// * `id` - The unique identifier of the `User` to retrieve.
    ///
    /// # Returns
    /// * `Result<Option<User>, sqlx::error::Error>` - A result containing an `Option` of `User`,
    ///   which will be `Some(User)` if found, or `None` if no record is found.
    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<User>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, datetime_created, password, datetime_confirmed, datetime_deactivated, datetime_deleted, auth_identity_id, email_id, pid_id, firstname_id, lastname_id
            FROM user
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Fetches a `User` record by its unique `email_id`.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool to query from.
    /// * `id` - The unique identifier of the user's email to retrieve the `User`.
    ///
    /// # Returns
    /// * `Result<Option<User>, sqlx::error::Error>` - A result containing an `Option` of `User`,
    ///   which will be `Some(User)` if found, or `None` if no record is found.
    pub async fn get_by_email_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<User>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, datetime_created, password, datetime_confirmed, datetime_deactivated, datetime_deleted, auth_identity_id, email_id, pid_id, firstname_id, lastname_id
            FROM user
            WHERE email_id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Fetches a `User` record by its unique `auth_identity_id`.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool to query from.
    /// * `id` - The unique identifier of the user's authentication identity to retrieve the `User`.
    ///
    /// # Returns
    /// * `Result<Option<User>, sqlx::error::Error>` - A result containing an `Option` of `User`,
    ///   which will be `Some(User)` if found, or `None` if no record is found.
    pub async fn get_by_auth_identity_id(
        pool: &Pool<MySql>,
        id: u64,
    ) -> Result<Option<User>, sqlx::error::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, datetime_created, password, datetime_confirmed, datetime_deactivated, datetime_deleted, auth_identity_id, email_id, pid_id, firstname_id, lastname_id
            FROM user
            WHERE auth_identity_id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Updates the `auth_identity_id` for the current `User`.
    ///
    /// This method updates the `auth_identity_id` field of the user in the database based on the user's
    /// `id`. This can be used to change the user's authentication identity, for example, if the user
    /// changes their login credentials or switches authentication providers.
    ///
    /// # Arguments
    ///
    /// * `tx` - The SQL transaction to execute the update query.
    /// * `user_auth_identity_id` - The new authentication identity ID to assign to the user.
    ///
    /// # Returns
    ///
    /// * `Result<(), sqlx::error::Error>` - A result indicating whether the update operation was successful.
    ///   If successful, it returns `Ok(())`. If an error occurs, it returns the error encountered.
    ///
    /// # Example
    ///
    /// ```rust
    /// let user = get_user_by_id(&pool, user_id).await?;
    /// user.update_auth_identity_id(&mut tx, new_auth_identity_id).await?;
    /// ```
    pub async fn update_auth_identity_id(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user_auth_identity_id: u64,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            r#"
            UPDATE user
            SET auth_identity_id = ?
            WHERE id = ?
            "#,
            user_auth_identity_id,
            self.id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
