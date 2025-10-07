use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySql, Transaction};

#[derive(Debug, Serialize, FromRow)]
pub struct OrganisationPIDModel {
    pub id: u64,
    pub datetime_created: NaiveDateTime,
    pub value: String,
}

impl OrganisationPIDModel {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        value: &str,
    ) -> Result<OrganisationPIDModel, sqlx::error::Error> {
        sqlx::query!(
            r#"
            INSERT INTO organisation_pid (value)
            VALUES (?)
            "#,
            value
        )
        .execute(&mut **tx)
        .await?;

        let row = sqlx::query_as!(
            OrganisationPIDModel,
            r#"
            SELECT id, datetime_created, value
            FROM board_pid
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }
}
