use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct BoardPid {
    pub id: u64,
    pub datetime_created: DateTime<Utc>,
    pub value: String,
}
