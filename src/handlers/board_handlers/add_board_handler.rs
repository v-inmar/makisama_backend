use actix_web::HttpMessage;
use actix_web::{HttpRequest, Responder, http::StatusCode, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::board_pid_model::BoardPid;
use crate::services::board_service::create_board;
use crate::utils::handler_utils;
use crate::utils::json_response_utils::JsonGeneralResponse;

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
    if json_data.name.len() < 1 || json_data.name.len() > 128 {
        return JsonGeneralResponse::make_response(
            &req,
            &StatusCode::BAD_REQUEST,
            "Name must be between 1 and 128 characters",
        );
    }

    if let Some(desc) = json_data.description.clone() {
        if desc.len() > 10000 {
            // restrict length to 10k characters, to avoid issues with encoding overflow
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::BAD_REQUEST,
                "Description must not exceed 10,000 characters",
            );
        }
    }

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

    // check if board name is already taken and in use
    // match BoardName::get_by_name(&pool, &name).await {
    //     Err(e) => {
    //         log::error!("{}", e);
    //         return JsonGeneralResponse::make_response(
    //             &req,
    //             &StatusCode::INTERNAL_SERVER_ERROR,
    //             "Server error, try again later",
    //         );
    //     }
    //     Ok(Some(bn)) => match Board::get_by_name_id(&pool, bn.id).await {
    //         Err(e) => {
    //             log::error!("{}", e);
    //             return JsonGeneralResponse::make_response(
    //                 &req,
    //                 &StatusCode::INTERNAL_SERVER_ERROR,
    //                 "Server error, try again later",
    //             );
    //         }
    //         Ok(Some(_)) => {
    //             return JsonGeneralResponse::make_response(
    //                 &req,
    //                 &StatusCode::CONFLICT,
    //                 "Name already in use",
    //             );
    //         }
    //         Ok(None) => {}
    //     },
    //     Ok(None) => {}
    // }

    // Call add new board service
    match create_board(&pool, user.id, &json_data).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(board) => match BoardPid::get_by_id(&pool, board.pid_id).await {
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
            Ok(Some(bp)) => match req.url_for("get_board", &[&bp.value]) {
                Err(e) => {
                    log::error!("{}", e);
                    let url = format!("/boards/{}", &bp.value);
                    return JsonGeneralResponse::make_response(&req, &StatusCode::CREATED, &url);
                }
                Ok(url) => {
                    return JsonGeneralResponse::make_response(
                        &req,
                        &StatusCode::CREATED,
                        &url.as_str(),
                    );
                }
            },
        },
    }
}
