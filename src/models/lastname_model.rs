use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Lastname {
    pub id: i64,
    pub value: String,
    pub datetime_created: DateTime<Utc>,
}
