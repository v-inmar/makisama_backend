use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, http::StatusCode, web};
use sqlx::MySqlPool;

use crate::{
    models::{board_model::Board, board_pid_model::BoardPid, board_user_model::BoardUser},
    services::board_service,
    utils::{handler_utils, json_response_utils::JsonGeneralResponse},
};

pub async fn delete_board(
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
        Ok(Some(bu)) => {
            // only owners can delete boards
            if bu.is_owner == false {
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::NOT_FOUND,
                    "Resource not found",
                );
            } else {
                match board_service::update_board_datetime_delete(&pool, &board).await {
                    Err(e) => {
                        log::error!("{}", e);
                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error, try again later",
                        );
                    }
                    Ok(result) => {
                        if result {
                            return HttpResponse::NoContent().finish();
                        } else {
                            return JsonGeneralResponse::make_response(
                                &req,
                                &StatusCode::INTERNAL_SERVER_ERROR,
                                "Server error, try again later",
                            );
                        }
                    }
                }
            }
        }
    };
}
