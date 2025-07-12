use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub datetime_deactivated: Option<DateTime<Utc>>,
    pub datetime_deleted: Option<DateTime<Utc>>,
    pub datetime_created: DateTime<Utc>,
}
