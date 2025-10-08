use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Transaction};

#[derive(Debug, FromRow, Serialize)]
pub struct OrganisationModel {
    pub id: u64,
    pub datetime_created: NaiveDateTime,
    pub name: String,
    pub description: String,
    pub datetime_deleted: Option<NaiveDateTime>,
    pub pid_id: u64,
}

impl OrganisationModel {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        name: &str,
        description: &str,
        pid_id: u64,
    ) -> Result<OrganisationModel, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO organisation (name, description, pid_id)
            VALUES (?, ?, ?)
            "#,
            name,
            description,
            pid_id
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            OrganisationModel,
            r#"
            SELECT 
                id, 
                datetime_created, 
                name, 
                description, 
                datetime_deleted, 
                pid_id
            FROM organisation 
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }
}
