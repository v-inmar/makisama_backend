use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::user_auth_identity_model::UserAuthIdentity;
use crate::models::user_email_model::UserEmail;
use crate::models::user_model::User;
use crate::services::user_service::create_user;
use crate::utils::json_response_utils::{JsonGeneralResponse, JsonJwtResponse};
use crate::utils::jwt_utils;
use crate::utils::string_utils::{is_alphabet_only, is_email_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequestData {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub repeat: String,
}

pub async fn register(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    json_data: web::Json<RegisterRequestData>,
) -> impl Responder {
    // ---- Request Input Data Validation ----

    // -- Use this for now but change it in the future for more robust validation
    // validation
    // firstname
    if is_alphabet_only(&json_data.firstname) == false {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Firstname must only contain Alphabet characters",
        );
    }

    if json_data.firstname.len() < 1 || json_data.firstname.len() > 64 {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Firstname lenght must be between 1 and 64 characters",
        );
    }

    // lastname
    if is_alphabet_only(&json_data.lastname) == false {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Lastname must only contain Alphabet characters",
        );
    }

    if json_data.lastname.len() < 1 || json_data.lastname.len() > 64 {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Lastname lenght must be between 1 and 64 characters",
        );
    }

    // email
    if is_email_format(&json_data.email) == false {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Email address has an invalid format",
        );
    }

    // password
    if json_data.password.len() < 8 {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Password must be atleast 8 characters long",
        );
    }

    // repeat password
    if json_data.repeat != json_data.password {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Password did not match",
        );
    }

    // ---- User Validation ----

    // Check email
    match UserEmail::get_by_value(&pool, &json_data.email.trim().to_lowercase()).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(ue)) => {
            // Check if email is in use
            match User::get_by_email_id(&pool, ue.id).await {
                Err(e) => {
                    log::error!("{}", e);
                    return JsonGeneralResponse::make_response(
                        &req,
                        &StatusCode::INTERNAL_SERVER_ERROR,
                        "Server error, try again later",
                    );
                }
                Ok(Some(_)) => {
                    return JsonGeneralResponse::make_response(
                        &req,
                        &StatusCode::CONFLICT,
                        "Email address is already in use",
                    );
                }
                Ok(None) => {}
            }
        }
        Ok(None) => {}
    }

    // ----- Create User -----
    let user = match create_user(&pool, &json_data).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(u) => u,
    };

    // ----- Create JWTokens -------

    // get auth id
    let user_ath_identity = match UserAuthIdentity::get_by_id(&pool, user.auth_identity_id).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(None) => {
            log::error!("Unable to get user auth identity id from newly created user");
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(uai)) => uai,
    };

    // generate tokens
    let access_token = match jwt_utils::generate_access_token(&user_ath_identity.value) {
        Ok(token) => token,
        Err(e) => {
            log::error!(
                "Unable to generate access token for newly created user. {}",
                e
            );
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
    };

    let refresh_token = match jwt_utils::generate_refresh_token(&user_ath_identity.value) {
        Ok(token) => token,
        Err(e) => {
            log::error!(
                "Unable to generate refresh token for newly created user. {}",
                e
            );
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
    };

    // -- Generate and reposne with jwt access token and cookie refresh token
    JsonJwtResponse::make_response(&req, &StatusCode::CREATED, &access_token, &refresh_token)
}
