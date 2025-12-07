use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::user_models::user_authid_model::UserAuthidModel;
use crate::models::user_models::user_email_model::UserEmailModel;
use crate::models::user_models::user_model::UserModel;
use crate::services::user_service::UserService;
use crate::utils::json_response_utils::{JsonGeneralResponse, JsonJwtResponse};
use crate::utils::jwt_utils;

use crate::utils::custom_validation_utils::{validate_email, validate_name};

// Data to be expected with the request
// #[derive(Debug, Deserialize, Serialize, Validate)]
// pub struct RegisterRequestData {
//     #[validate(custom(function = "validate_name"))]
//     pub firstname: String,

//     #[validate(custom(function = "validate_name"))]
//     pub lastname: String,

//     #[validate(custom(function = "validate_email"))]
//     pub email: String,

//     #[validate(length(min = 8, max = 255))]
//     pub password: String,

//     #[validate(length(min = 8, max = 255))]
//     pub repeat: String,
// }

pub async fn register(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    json_data: web::Json<RegisterRequestData>,
) -> impl Responder {
    // ------------------------------------------- Checking Request ----------------------------------------

    // validate the incoming data
    match json_data.validate() {
        Ok(_) => (),
        Err(e) => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::BAD_REQUEST,
                &e.to_string().as_str(),
            );
        }
    }

    // check password == repeat
    if json_data.password != json_data.repeat {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Password and Repeat did not match",
        );
    }

    // check email if in use
    match UserEmailModel::get_by_value(&pool, &json_data.email).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(ue)) => {
            // email exist, chech if its in use
            match UserModel::get_by_email_id(&pool, ue.id).await {
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

    let user_obj = match UserService::create_user(&pool, &json_data).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(user) => user,
    };

    // --------------------------------- Making the Response --------------------------------

    // ----- Create JWTokens -------
    // get auth id
    let user_authid_obj = match UserAuthidModel::get_by_id(&pool, user_obj.authid_id).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(None) => {
            log::error!(
                "Unable to get user auth identity id from newly created user. user id: {}",
                user_obj.id
            );
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(uam)) => uam,
    };

    // generate access tokens
    let access_token = match jwt_utils::generate_access_token(&user_authid_obj.value) {
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

    // generate refresh token
    let refresh_token = match jwt_utils::generate_refresh_token(&user_authid_obj.value) {
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
