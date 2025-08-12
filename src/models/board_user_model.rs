use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct UserBoard {
    pub id: u64,
    pub datetime_created: DateTime<Utc>,
    pub board_id: u64,
    pub user_id: u64,
    pub is_owner: bool,
    pub is_admin: bool,
}
