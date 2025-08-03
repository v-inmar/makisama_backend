use actix_web::HttpMessage;
use actix_web::{HttpRequest, Responder, http::StatusCode, web};
use sqlx::MySqlPool;

use crate::models::board_member_model::BoardMember;
use crate::models::board_model::Board;
use crate::utils::handler_utils::get_user_by_auth_identity;
use crate::utils::json_response_utils::JsonGeneralResponse;

pub async fn get_board(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    name: web::Path<String>,
) -> impl Responder {
    let board_name = name.as_str().to_lowercase();

    // get the board
    let board = match Board::get_by_name(&pool, &board_name).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later.",
            );
        }
        Ok(None) => {
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

    let user = match get_user_by_auth_identity(&pool, &at_sub).await {
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

    // check if user is a board member
    match BoardMember::get_board_members_by_user_id_and_board_id(&pool, user.id, board.id).await {
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
    }

    // get the user from req.extension
    JsonGeneralResponse::make_response(&req, &StatusCode::OK, &board.name)
}
