use crate::handlers::auth_handlers::RegisterRequestData;
use crate::models::user_models::user_authid_model::UserAuthidModel;
use crate::models::user_models::user_email_model::UserEmailModel;
use crate::models::user_models::user_model::UserModel;
use crate::models::user_models::user_name_model::UserNameModel;
use crate::models::user_models::user_pid_model::UserPidModel;
use crate::utils::bcrypt_utils::make_hash;
use crate::utils::string_utils::random_alphanumeric;

// use chrono::{Duration, Utc};
use sqlx::{MySql, Pool};

pub struct UserService {}

impl UserService {
    pub async fn create_user(
        pool: &Pool<MySql>,
        data: &RegisterRequestData,
    ) -> Result<UserModel, Box<dyn std::error::Error>> {
        // create transaction instance
        let mut tx = pool.begin().await?;

        // create email
        let user_email_obj = match UserEmailModel::get_by_value(&pool, &data.email).await? {
            Some(e) => {
                // check if its being used
                if let Some(_) = UserModel::get_by_email_id(&pool, e.id).await? {
                    let err_msg = format!("Email: {} is already in use", data.email);
                    log::error!("{}", err_msg);
                    return Err(err_msg.into());
                } else {
                    e // not in used
                }
            }
            None => UserEmailModel::new(&mut tx, &data.email).await?,
        };

        // create authid
        let user_authid_obj: UserAuthidModel;
        let mut authid_counter: i32 = 0;
        loop {
            // will only try 5 times
            if authid_counter == 5 {
                let err_msg = String::from(
                    "Error while creating UserAuthidModel. Try limit has been reached",
                );
                log::error!("{}", err_msg);
                return Err(err_msg.into());
            }

            authid_counter += 1;
            let authid_value = random_alphanumeric(32);
            match UserAuthidModel::get_by_value(&pool, &authid_value).await? {
                Some(_) => continue,
                None => {
                    user_authid_obj = UserAuthidModel::new(&mut tx, &authid_value).await?;
                    break;
                }
            }
        }

        // create pid
        let user_pid_obj: UserPidModel;
        let mut pid_counter = 0;
        loop {
            if pid_counter == 5 {
                let err_msg =
                    String::from("Error while creating UserPidModel. Try limit has been reached");
                log::error!("{}", err_msg);
                return Err(err_msg.into());
            }

            pid_counter += 1;
            let pid_value = random_alphanumeric(32);
            match UserPidModel::get_by_value(&pool, &pid_value).await? {
                Some(_) => continue,
                None => {
                    user_pid_obj = UserPidModel::new(&mut tx, &pid_value).await?;
                    break;
                }
            }
        }

        // create firstname
        let user_fname_obj = match UserNameModel::get_by_value(&pool, &data.firstname).await? {
            Some(f) => f,
            None => UserNameModel::new(&mut tx, &data.firstname).await?,
        };

        // create lastname
        let user_lname_obj: UserNameModel =
            match UserNameModel::get_by_value(&pool, &data.lastname).await? {
                Some(l) => l,
                None => UserNameModel::new(&mut tx, &data.lastname).await?,
            };

        // hash the password
        let hashed_pw = make_hash(&data.password)?;

        // create the user
        let user_obj: UserModel = UserModel::new(
            &mut tx,
            &hashed_pw,
            user_fname_obj.id,
            user_lname_obj.id,
            user_email_obj.id,
            user_pid_obj.id,
            user_authid_obj.id,
        )
        .await?;

        tx.commit().await?;

        Ok(user_obj)
    }
}
