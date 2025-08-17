use actix_web::HttpMessage;
use actix_web::{HttpRequest, Responder, http::StatusCode, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::board_model::Board;
use crate::models::board_name_model::BoardName;
use crate::models::user_auth_identity_model::AuthIdentity;
use crate::models::user_model::User;
use crate::services::board_service::create_board_service;
use crate::utils::json_response_utils::JsonGeneralResponse;
use crate::utils::string_utils::is_alphanumeric_or_underscore;
use crate::utils::string_utils::is_first_character_underscore;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddBoardRequestData {
    pub name: String,
    pub description: Option<String>,
}

pub async fn add_board(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    json_data: web::Json<AddBoardRequestData>,
) -> impl Responder {
    // ** this for now, replace with validator **

    let name = json_data.name.to_lowercase().to_string();

    // check length (5 - 128)
    if name.len() < 5 || name.len() > 128 {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Must be between 5 and 128 characters",
        );
    }

    if is_alphanumeric_or_underscore(&name) == false {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Only alphanumeric and underscore",
        );
    }

    if is_first_character_underscore(&name) == true {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Must not start with underscore",
        );
    }

    /*
       DOUBLE CHECK AUTHORIZATION
    */
    // get user id from the req extension
    let at_sub = match req.extensions().get::<String>() {
        Some(sub) => sub.clone(),
        None => {
            // access toke sub (which is the user auth id) is not present
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                "Access token is required",
            );
        }
    };

    // anything other than the auth identity object, something went wrong so return 500
    let auth_identity = match AuthIdentity::get_by_value(&pool, &at_sub).await {
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
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(ai)) => ai,
    };

    // get the user using auth identity
    let user: User = match User::get_user_by_auth_identity_id(&pool, auth_identity.id).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(u)) => u,
        Ok(None) => {
            log::error!("Unable to get user using auth id value");
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
    };

    // check if board name is already taken and in use
    match BoardName::get_by_name(&pool, &name).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(bn)) => match Board::get_by_name_id(&pool, bn.id).await {
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
                    "Name already in use",
                );
            }
            Ok(None) => {}
        },
        Ok(None) => {}
    }

    // Call add new board service
    match create_board_service(&pool, &name, user.id).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(_) => {
            // it is safe to use the client-sent name value since it can only contain alphanumeric and underscore
            // it was checked above
            let board_url = match req.url_for("get_board", &[&name]) {
                Ok(bu) => bu,
                Err(e) => {
                    log::error!("{}", e);

                    let url = format!("/boards/{}", &name);
                    return JsonGeneralResponse::make_response(&req, &StatusCode::CREATED, &url);
                }
            };

            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::CREATED,
                &board_url.as_str(),
            );
        }
    }
}
