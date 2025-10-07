use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Transaction};

#[derive(Serialize, FromRow, Debug)]
pub struct OrganisationModel {
    pub id: u64,
    pub datetime_created: NaiveDateTime,
    pub name: String,
    pub description: String,
    pub datetime_deleted: Option<NaiveDateTime>,
    pub pid_id: u64,
}
