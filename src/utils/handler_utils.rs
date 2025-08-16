use sqlx::{MySql, Pool};

use crate::models::user_auth_identity_model::UserAuthIdentity;
use crate::models::user_model::User;

pub async fn get_user_by_auth_identity(
    pool: &Pool<MySql>,
    auth_identity: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    match UserAuthIdentity::get_by_value(&pool, &auth_identity).await {
        Err(e) => {
            log::error!("{}", e);
            return Err(e.into());
        }
        Ok(None) => {
            return Ok(None);
        }
        Ok(Some(ai)) => match User::get_by_auth_identity_id(&pool, ai.id).await {
            Err(e) => {
                log::error!("{}", e);
                return Err(e.into());
            }
            Ok(None) => {
                return Ok(None);
            }
            Ok(Some(u)) => {
                return Ok(Some(u));
            }
        },
    }
}
