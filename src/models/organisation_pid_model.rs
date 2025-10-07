use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct OrganisationPIDModel {
    pub id: u64,
    pub datetime_created: NaiveDateTime,
    pub value: String,
}
