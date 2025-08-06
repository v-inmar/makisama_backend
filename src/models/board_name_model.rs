use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct BoardName {
    pub id: i64,
    pub datetime_created: DateTime<Utc>,
    pub name: String,
}
