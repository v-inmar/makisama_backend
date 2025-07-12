use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct UserAuthToken {
    pub id: i64,
    pub token: String,
    pub datetime_created: DateTime<Utc>,
}
