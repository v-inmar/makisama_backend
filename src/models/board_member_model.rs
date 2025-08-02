use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct BoardMember {
    pub id: i64,
    pub datetime_created: DateTime<Utc>,
    pub board_id: i64,
    pub user_id: i64,
    pub is_owner: bool,
    pub is_admin: bool,
}
