use bcrypt::{DEFAULT_COST, hash};
use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::{
    handlers::auth_handlers::register_handler::RegisterRequestData,
    models::{
        auth_identity_model::AuthIdentity, firstname_model::Firstname, lastname_model::Lastname,
        user_model::User,
    },
};

pub async fn register_new_user(
    pool: &Pool<MySql>,
    data: &RegisterRequestData,
) -> Result<User, Box<dyn std::error::Error>> {
    // begin transaction
    let mut tx = pool.begin().await?;

    // create auth identity
    let auth_identity: AuthIdentity;
    let mut counter = 0;
    loop {
        if counter == 6 {
            let err_msg = "Error while creating AuthIdentity. Try limit has been reached";
            log::error!("{}", err_msg);
            return Err(err_msg.into());
        }

        counter += 1;
        let auth_identity_value = Uuid::new_v4().to_string();
        match AuthIdentity::get_by_value(pool, &auth_identity_value).await? {
            Some(_) => continue,
            None => {
                auth_identity = AuthIdentity::new(&mut tx, &auth_identity_value).await?;
                break;
            }
        }
    }

    // firstname
    let firstname = match Firstname::get_by_value(pool, &data.firstname).await? {
        Some(fname) => fname,
        None => Firstname::new(&mut tx, &data.firstname).await?,
    };

    // lastname
    let lastname = match Lastname::get_by_value(pool, &data.lastname).await? {
        Some(lname) => lname,
        None => Lastname::new(&mut tx, &data.lastname).await?,
    };

    // user
    let hashed_password = hash(data.password.clone(), DEFAULT_COST)?;
    let new_user = User::new(
        &mut tx,
        &data.username,
        &data.email,
        &hashed_password,
        firstname.id,
        lastname.id,
        auth_identity.id,
    )
    .await?;

    tx.commit().await?;

    Ok(new_user)
    // user
    // return user
}
