use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct RevokedToken {
    pub id: u64,
    pub value: String,
    pub datetime_ttl: DateTime<Utc>,
    pub datetime_created: DateTime<Utc>,
}
