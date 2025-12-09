use chrono::NaiveDateTime;
use sqlx::MySql;
use sqlx::Pool;

use crate::models::revoked_token_models::revoked_token_model::RevokedTokenModel;

pub struct AuthService {}

impl AuthService {
    pub async fn create_revoked(
        pool: &Pool<MySql>,
        token: &str,
        datetime_ttl: &NaiveDateTime,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = pool.begin().await?;
        RevokedTokenModel::new(&mut tx, token, datetime_ttl).await?;
        tx.commit().await?;
        Ok(())
    }
}
