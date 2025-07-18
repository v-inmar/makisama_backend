use actix_web::{HttpRequest, Responder, http::StatusCode, post, web};

use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::{
    models::{auth_identity_model::AuthIdentity, user_model::User},
    utils::{
        bcrypt_utils::is_matched,
        json_response_utils::JsonGeneralResponse,
        json_response_utils::JsonJwtResponse,
        jwt_utils::{generate_access_token, generate_refresh_token},
    },
};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    form: web::Form<LoginForm>,
) -> impl Responder {
    let email = &form.email;
    let password = &form.password;

    let user = match User::get_user_by_email(&pool, email).await {
        Err(e) => {
            log::error!("Unable to get user by email. {}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
        Ok(Some(user)) => user,
        Ok(None) => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                &String::from("Invalid email and/or password"),
            );
        }
    };

    let hashed = &user.password;
    match is_matched(&password, hashed) {
        Err(e) => {
            log::error!("Unable to check password. {}", e);
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
                match AuthIdentity::get_by_id(&pool, user.auth_identity_id).await {
                    Err(e) => {
                        log::error!("Unable to get auth identity. {}", e);
                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error. Try again later.",
                        );
                    }
                    Ok(None) => {
                        log::error!("No auth identity for user.");
                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error. Try again later.",
                        );
                    }
                    Ok(Some(aio)) => {
                        let access_token = match generate_access_token(&aio.value) {
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

                        let refresh_token = match generate_refresh_token(&aio.value) {
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
    }
}
