use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::{
    handlers::auth_handlers::register_handler::RegisterRequestData,
    models::{
        user_auth_identity_model::UserAuthIdentity, user_email_model::UserEmail,
        user_firstname_model::UserFirstname, user_lastname_model::UserLastname, user_model::User,
        user_pid_model::UserPid,
    },
    utils::bcrypt_utils::make_hash,
};

/// Creates a new user in the system.
///
/// This function performs the following steps to create a new user:
/// 1. Generates a unique `UserAuthIdentity` (UUID) and ensures it does not exist in the database.
///    If a conflict occurs, it retries up to 6 times.
/// 2. Generates a unique `UserPid` (UUID without hyphens) and ensures it does not exist in the database.
///    If a conflict occurs, it retries up to 6 times.
/// 3. Checks if the provided email is already registered and in use. If it is not in use, it creates a new `UserEmail`.
/// 4. If the provided `firstname` does not already exist, it creates a new `UserFirstname`.
/// 5. If the provided `lastname` does not already exist, it creates a new `UserLastname`.
/// 6. Hashes the provided password using bcrypt.
/// 7. Creates a new user record in the `User` table with the generated and/or existing values.
/// 8. Commits the transaction to the database.
///
/// # Arguments
///
/// * `pool`: A reference to the database connection pool.
/// * `data`: A reference to the `RegisterRequestData` containing user input (email, firstname, lastname, and password).
///
/// # Returns
///
/// * `Result<User, Box<dyn std::error::Error>>`: Returns the created user object on success, or an error message if any step fails.
pub async fn create_user(
    pool: &Pool<MySql>,
    data: &RegisterRequestData,
) -> Result<User, Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;

    // -- auth identity
    let user_auth_identity_obj: UserAuthIdentity;
    let mut counter = 0;
    loop {
        if counter == 6 {
            let err_msg = "Error while creating UserAuthIdentity. Try limit has been reached";
            log::error!("{}", err_msg);
            return Err(err_msg.into());
        }

        counter += 1;
        let auth_identity_value = Uuid::new_v4().to_string(); // can be upto 36 chars long
        match UserAuthIdentity::get_by_value(&pool, &auth_identity_value).await? {
            Some(_) => continue,
            None => {
                user_auth_identity_obj =
                    UserAuthIdentity::new(&mut tx, &auth_identity_value).await?;
                break;
            }
        }
    }

    // pid
    let user_pid_obj: UserPid;
    let mut counter_pid = 0;
    loop {
        if counter_pid == 6 {
            let err_msg = "Error while creating UserPid. Try limit has been reached";
            log::error!("{}", err_msg);
            return Err(err_msg.into());
        }

        counter_pid += 1;
        let uuid_str = Uuid::new_v4().to_string();
        let uuid_no_hyphens = uuid_str.replace("-", "");
        let pid_value = &uuid_no_hyphens[0..32];
        match UserPid::get_by_value(&pool, pid_value).await? {
            Some(_) => continue,
            None => {
                user_pid_obj = UserPid::new(&mut tx, pid_value).await?;
                break;
            }
        }
    }

    // -- email
    let user_email_obj: UserEmail;
    match UserEmail::get_by_value(&pool, &data.email.to_lowercase()).await? {
        Some(email_obj) => {
            // already exist. check if it is used
            if let Some(_) = User::get_by_email_id(&pool, email_obj.id).await? {
                let err_msg = format!("Email: {} is already in use", data.email);
                log::error!("{}", err_msg);
                return Err(err_msg.into());
            } else {
                user_email_obj = email_obj; // email exist but not in use
            }
        }
        None => {
            user_email_obj = UserEmail::new(&mut tx, &data.email).await?;
        }
    }

    // -- firstname
    let user_firstname_obj = match UserFirstname::get_by_value(&pool, &data.firstname).await? {
        Some(f) => f,
        None => UserFirstname::new(&mut tx, &data.firstname).await?,
    };

    // -- lastname
    let user_lastname_obj = match UserLastname::get_by_value(&pool, &data.lastname).await? {
        Some(l) => l,
        None => UserLastname::new(&mut tx, &data.lastname).await?,
    };

    // -- hash the password using bcrypt
    let hashed_password = make_hash(&data.password)?;

    // -- user
    let user_obj = User::new(
        &mut tx,
        &hashed_password,
        user_auth_identity_obj.id,
        user_email_obj.id,
        user_pid_obj.id,
        user_firstname_obj.id,
        user_lastname_obj.id,
    )
    .await?;

    tx.commit().await?;

    Ok(user_obj)
}

/// Updates a user's authentication identity by generating a new `UserAuthIdentity` and associating it with the user.
///
/// This method creates a new `UserAuthIdentity` (ensuring its uniqueness), and then updates the `auth_identity_id`
/// of the given user with the new `UserAuthIdentity`. It wraps the operation in a database transaction to ensure
/// atomicity. If the process of generating a unique `UserAuthIdentity` fails after several attempts, an error is returned.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool for executing queries.
/// * `user` - The user whose `auth_identity_id` is being updated.
///
/// # Returns
///
/// * `Result<(), sqlx::error::Error>` - A result indicating success or failure of the operation. If the update is
///   successful, it returns `Ok(())`. If an error occurs at any point, it returns an error with relevant details.
///
/// # Behavior
///
/// 1. The function attempts to generate a unique `UserAuthIdentity` ID (using a UUID) and ensures that the ID does not
///    already exist in the database by querying `UserAuthIdentity::get_by_value`.
/// 2. If a unique `UserAuthIdentity` is found, it is created and associated with the user by updating their `auth_identity_id`.
/// 3. The operation is wrapped in a transaction, ensuring atomicity.
/// 4. If the system cannot generate a unique `UserAuthIdentity` after 6 attempts, it logs an error and returns a failure.
///
/// # Example
///
/// ```rust
/// let result = update_user_auth_identity_id(&pool, &user).await;
/// match result {
///     Ok(_) => println!("User auth identity updated successfully."),
///     Err(e) => eprintln!("Error updating user auth identity: {:?}", e),
/// }
/// ```
pub async fn update_user_auth_identity_id(
    pool: &Pool<MySql>,
    user: &User,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;

    // -- auth identity
    let user_auth_identity: UserAuthIdentity;
    let mut counter = 0;
    loop {
        if counter == 6 {
            let err_msg = "Error while creating UserAuthIdentity. Try limit has been reached";
            log::error!("{}", err_msg);
            return Err(err_msg.into());
        }

        counter += 1;
        let auth_identity_value = Uuid::new_v4().to_string(); // can be up to 36 chars long
        match UserAuthIdentity::get_by_value(&pool, &auth_identity_value).await? {
            Some(_) => continue,
            None => {
                user_auth_identity = UserAuthIdentity::new(&mut tx, &auth_identity_value).await?;
                break;
            }
        }
    }

    user.update_auth_identity_id(&mut tx, user_auth_identity.id)
        .await?;

    tx.commit().await?;

    Ok(())
}

// use chrono::{Duration, Utc};
// use sqlx::{MySql, Pool};
// use uuid::Uuid;

// use crate::{
//     handlers::auth_handlers::register_handler::RegisterRequestData,
//     models::{
//         user_auth_identity_model::AuthIdentity, user_firstname_model::Firstname,
//         user_lastname_model::Lastname, user_model::User,
//     },
//     utils::bcrypt_utils::make_hash,
// };

// pub async fn register_new_user(
//     pool: &Pool<MySql>,
//     data: &RegisterRequestData,
// ) -> Result<User, Box<dyn std::error::Error>> {
//     // begin transaction
//     let mut tx = pool.begin().await?;

//     // create auth identity
//     let auth_identity: AuthIdentity;
//     let mut counter = 0;
//     loop {
//         if counter == 6 {
//             let err_msg = "Error while creating AuthIdentity. Try limit has been reached";
//             log::error!("{}", err_msg);
//             return Err(err_msg.into());
//         }

//         counter += 1;
//         let auth_identity_value = Uuid::new_v4().to_string();
//         match AuthIdentity::get_by_value(pool, &auth_identity_value).await? {
//             Some(_) => continue,
//             None => {
//                 auth_identity = AuthIdentity::new(&mut tx, &auth_identity_value).await?;
//                 break;
//             }
//         }
//     }

//     // firstname
//     let firstname = match Firstname::get_by_value(pool, &data.firstname).await? {
//         Some(fname) => fname,
//         None => Firstname::new(&mut tx, &data.firstname).await?,
//     };

//     // lastname
//     let lastname = match Lastname::get_by_value(pool, &data.lastname).await? {
//         Some(lname) => lname,
//         None => Lastname::new(&mut tx, &data.lastname).await?,
//     };

//     // user
//     let hashed_password = make_hash(&data.password)?;
//     let new_user = User::new(
//         &mut tx,
//         &data.username,
//         &data.email,
//         &hashed_password,
//         firstname.id,
//         lastname.id,
//         auth_identity.id,
//     )
//     .await?;

//     tx.commit().await?;

//     Ok(new_user)
//     // user
//     // return user
// }

// pub async fn create_new_auth_id(
//     pool: &Pool<MySql>,
//     user: &User,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // begin transaction
//     let mut tx = pool.begin().await?;

//     match AuthIdentity::get_by_id(pool, user.auth_identity_id).await? {
//         None => {
//             return Err(format!(
//                 "Unable to get Auth Identity with matching id: {}",
//                 user.auth_identity_id
//             )
//             .into());
//         }
//         Some(aio) => {
//             // add ttl to the auth id object (1 year)
//             // value cant be used for 1 year
//             let ttl = Utc::now() + Duration::days(365);
//             aio.update_ttl(&mut tx, &ttl.naive_utc()).await?;

//             // new auth id
//             let new_auth_identity: AuthIdentity;
//             let mut counter = 0;
//             loop {
//                 if counter == 6 {
//                     let err_msg = "Error while creating AuthIdentity. Try limit has been reached";
//                     log::error!("{}", err_msg);
//                     return Err(err_msg.into());
//                 }

//                 counter += 1;
//                 let auth_identity_value = Uuid::new_v4().to_string();
//                 match AuthIdentity::get_by_value(pool, &auth_identity_value).await? {
//                     Some(_) => continue,
//                     None => {
//                         new_auth_identity =
//                             AuthIdentity::new(&mut tx, &auth_identity_value).await?;
//                         break;
//                     }
//                 }
//             }
//             user.update_auth_identity_id(&mut tx, new_auth_identity.id)
//                 .await?;

//             tx.commit().await?;

//             Ok(())
//         }
//     }
// }
