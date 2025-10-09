use actix_web::{HttpMessage, HttpRequest, Responder, http::StatusCode, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::utils::{handler_utils, json_response_utils::JsonGeneralResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrganisationRequestData {
    pub name: String,
    pub description: String,
}

pub async fn add_organisation(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    json_data: web::Json<AddOrganisationRequestData>,
) -> impl Responder {
    // name must have length between 1 and 128
    if json_data.name.trim().len() < 1 || json_data.name.trim().len() > 128 {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "name must be between 1 and 128 characters",
        );
    }

    // description must have length between 1 and 128
    if json_data.description.trim().len() < 1 || json_data.description.trim().len() > 255 {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "description must be between 1 and 255 characters",
        );
    }

    // get user id from the req extension
    let at_sub: String = match req.extensions().get::<String>() {
        Some(sub) => sub.clone(),
        None => {
            // access token sub (which is the user auth id) is not present
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                "Access token is required",
            );
        }
    };

    // get current user
    let user = match handler_utils::get_user_by_auth_identity(&pool, &at_sub).await {
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
                "Access token is required",
            );
        }
        Ok(Some(u)) => u,
    };

    // Call add new organisation service

    // dummy return, remove this before finishing this handler
    JsonGeneralResponse::make_response(&req, &StatusCode::CREATED, "Just dummy response")
}
