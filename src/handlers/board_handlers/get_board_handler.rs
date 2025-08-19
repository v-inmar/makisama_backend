use actix_web::{HttpMessage, HttpResponse};
use actix_web::{HttpRequest, Responder, http::StatusCode, web};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::board_description_model::BoardDescription;
use crate::models::board_model::Board;
use crate::models::board_name_model::BoardName;
use crate::models::board_pid_model::BoardPid;
use crate::models::board_user_model::BoardUser;
use crate::utils::handler_utils;
use crate::utils::json_response_utils::{JsonGeneralResponse, RequestDetails, StatusDetails};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBoardResponseData {
    pub name: String,
    pub description: Option<String>,
    pub datetime_created: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBoardResponse {
    pub request_details: RequestDetails,
    pub status_details: StatusDetails,
    pub payload: GetBoardResponseData,
}

pub async fn get_board(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    pid: web::Path<String>,
) -> impl Responder {
    // board pid
    let board_pid = match BoardPid::get_by_value(&pool, &pid.as_str()).await {
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
                &StatusCode::NOT_FOUND,
                "Resource not found",
            );
        }
        Ok(Some(bp)) => bp,
    };

    // board
    let board = match Board::get_by_pid_id(&pool, board_pid.id).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(None) => {
            // returns 404 because pid can exist without a board
            // or board has been soft deleted
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::NOT_FOUND,
                "Resource not found",
            );
        }
        Ok(Some(b)) => b,
    };

    // Get user from auth identity
    let at_sub = match req.extensions().get::<String>() {
        Some(sub) => sub.clone(),
        None => {
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

    // check if user is board user
    match BoardUser::get_by_board_id_and_user_id(&pool, board.id, user.id).await {
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
                &StatusCode::NOT_FOUND,
                "Resource not found",
            );
        }
        Ok(Some(_)) => {}
    };

    // get board name
    let board_name = match BoardName::get_by_id(&pool, board.name_id).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(None) => {
            // this should not happen
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(Some(bn)) => bn,
    };

    // Logic to fetch board description (simplified)
    let board_description: Option<BoardDescription> = if let Some(bdid) = board.description_id {
        match BoardDescription::get_by_id(&pool, bdid).await {
            Ok(Some(bd)) => Some(bd), // Only assign Some(bd) if found
            _ => None,                // Either error or no description found
        }
    } else {
        None
    };

    let resp_data = GetBoardResponseData {
        name: board_name.value,
        description: board_description.map(|bd| bd.value),
        datetime_created: board.datetime_created,
    };

    /*
        name
        description
        date created
    */

    let response = GetBoardResponse {
        request_details: RequestDetails::new(&req),
        status_details: StatusDetails::new(&StatusCode::OK),
        payload: resp_data,
    };

    // build the response
    let mut resp_builder = HttpResponse::build(StatusCode::OK);
    resp_builder.content_type("application/json");
    return resp_builder.json(response);
}
