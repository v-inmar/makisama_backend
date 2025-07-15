use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct AuthIdentity {
    pub id: i64,
    pub value: String,
    pub datetime_ttl: Option<DateTime<Utc>>, // ttl only gets populated once auth identity is not being used
    pub datetime_created: DateTime<Utc>,
}
