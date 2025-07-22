use chrono::NaiveDateTime;
use sqlx::{MySql, Pool};

use crate::models::revoked_token_model::RevokedToken;

pub async fn revoke_user_refresh_token(
    pool: &Pool<MySql>,
    token: &str,
    datetime_ttl: &NaiveDateTime,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;

    RevokedToken::new(&mut tx, token, datetime_ttl).await?;

    tx.commit().await?;

    Ok(())
}
