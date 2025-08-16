use actix_web::{HttpRequest, Responder, http::StatusCode, web};

use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::{
    models::{
        user_auth_identity_model::UserAuthIdentity, user_email_model::UserEmail, user_model::User,
    },
    utils::{
        bcrypt_utils::is_matched,
        json_response_utils::{JsonGeneralResponse, JsonJwtResponse},
        jwt_utils::{generate_access_token, generate_refresh_token},
    },
};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    form: web::Form<LoginForm>,
) -> impl Responder {
    let email = &form.email;
    let password = &form.password;

    // get email
    let user_email = match UserEmail::get_by_value(&pool, &email.trim().to_lowercase()).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(None) => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                &String::from("Invalid email and/or password"),
            );
        }
        Ok(Some(ue)) => ue,
    };

    // get user
    let user = match User::get_by_email_id(&pool, user_email.id).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(None) => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                &String::from("Invalid email and/or password"),
            );
        }
        Ok(Some(u)) => u,
    };

    match is_matched(&password, &user.password) {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
        Ok(matched) => {
            if !matched {
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::UNAUTHORIZED,
                    &String::from("Invalid email and/or password"),
                );
            } else {
                // everything checked out
                // now generate the tokens

                let user_auth_identity = match UserAuthIdentity::get_by_id(
                    &pool,
                    user.auth_identity_id,
                )
                .await
                {
                    Err(e) => {
                        log::error!("{}", e);
                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error. Try again later.",
                        );
                    }
                    Ok(None) => {
                        log::error!(
                            "Unable to get user auth identity id from a valid user. user id: {}",
                            user.id
                        );
                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error, try again later",
                        );
                    }
                    Ok(Some(uai)) => uai,
                };

                let access_token = match generate_access_token(&user_auth_identity.value) {
                    Ok(token) => token,
                    Err(e) => {
                        log::error!("Unable to generate access token user. {}", e);
                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error. Try again later.",
                        );
                    }
                };

                let refresh_token = match generate_refresh_token(&user_auth_identity.value) {
                    Ok(token) => token,
                    Err(e) => {
                        log::error!("Unable to generate refresh token for user. {}", e);

                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error. Try again later.",
                        );
                    }
                };

                return JsonJwtResponse::make_response(
                    &req,
                    &StatusCode::OK,
                    &access_token,
                    &refresh_token,
                );
            }
        }
    }
}
