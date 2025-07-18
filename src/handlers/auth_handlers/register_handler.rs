use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Responder, post, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::auth_identity_model::AuthIdentity;
use crate::models::user_model::User;
use crate::services::user_service::register_new_user;
use crate::utils::json_response_utils::{JsonGeneralResponse, JsonJwtResponse};
use crate::utils::jwt_utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequestData {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub repeat: String,
}

#[post("/register")]
pub async fn register(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    json_data: web::Json<RegisterRequestData>,
) -> impl Responder {
    // Check email
    match User::get_user_by_email(&pool, &json_data.email).await {
        Err(e) => {
            log::error!("Error while checking for email. {}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
        Ok(Some(_)) => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::CONFLICT,
                &String::from("Email address already in use"),
            );
            // return HttpResponse::Conflict().body("Email address already in use");
        }
        Ok(None) => (),
    }

    // Check username
    match User::get_user_by_username(&pool, &json_data.username).await {
        Err(e) => {
            log::error!("Error while checking for username. {}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
        Ok(Some(_)) => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::CONFLICT,
                &String::from("Username already in use"),
            );
            // return HttpResponse::Conflict().body("Username already in use");
        }
        Ok(None) => (),
    }

    // Check password and repeat matched
    if json_data.password != json_data.repeat {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            &String::from("Password did not match"),
        );
        // return HttpResponse::BadRequest().body("Password did not match");
    }

    // register new user
    let user = match register_new_user(&pool, &json_data).await {
        Err(e) => {
            log::error!("Error when trying to register new user. {}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
        Ok(user) => user,
    };

    // get auth identity
    let auth_identity = match AuthIdentity::get_by_id(&pool, user.auth_identity_id).await {
        Err(e) => {
            log::error!(
                "Error when trying to get auth identity of newly created user. {}",
                e
            );

            // user has been created so not really internal server error response anymore
            // try a better response
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
        Ok(Some(aio)) => aio,
        Ok(None) => {
            // this should never happen unless something wrong with db, connection or register
            log::error!("Empty row auth identity for newly created user.");

            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error. Try again later.",
            );
        }
    };

    // generate tokens
    let access_token = match jwt_utils::generate_access_token(&auth_identity.value) {
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

    let refresh_token = match jwt_utils::generate_refresh_token(&auth_identity.value) {
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

    JsonJwtResponse::make_response(&req, &StatusCode::CREATED, &access_token, &refresh_token)
}
