use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub username: String,
    pub datetime_deactivated: Option<DateTime<Utc>>,
    pub datetime_deleted: Option<DateTime<Utc>>,
    pub datetime_created: DateTime<Utc>,
    pub auth_identity_id: i64,
    pub firstname_id: i64,
    pub lastname_id: i64,
}

/*
id BIGINT AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    username VARCHAR(64) NOT NULL UNIQUE,
    datetime_deactivated TIMESTAMP DEFAULT NULL,
    datetime_deleted TIMESTAMP DEFAULT NULL,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    auth_identity_id BIGINT NOT NULL UNIQUE,
    username_id BIGINT NOT NULL UNIQUE,
    firstname_id BIGINT NOT NULL,
    lastname_id BIGINT NOT NULL,

*/
