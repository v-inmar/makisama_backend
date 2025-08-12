use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: u64,
    pub datetime_created: DateTime<Utc>,
    pub password: String,
    pub datetime_deactivated: Option<DateTime<Utc>>,
    pub datetime_deleted: Option<DateTime<Utc>>,
    pub auth_identity_id: u64,
    pub email_id: u64,
    pub pid_id: u64,
    pub firstname_id: u64,
    pub lastname_id: u64,
}
