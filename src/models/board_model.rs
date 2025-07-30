use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Board {
    pub id: i64,
    pub datetime_created: DateTime<Utc>,
    pub board_name_id: i64,
}
