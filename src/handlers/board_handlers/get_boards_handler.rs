use actix_web::HttpResponse;
use actix_web::{HttpMessage, HttpRequest, Responder, http::StatusCode, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::board_model::Board;
use crate::utils::json_response_utils::{RequestDetails, StatusDetails};
use crate::utils::{
    handler_utils::get_user_by_auth_identity, json_response_utils::JsonGeneralResponse,
};

use crate::models::board_member_model::BoardMember;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBoard {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBoardsResponse {
    pub request_details: RequestDetails,
    pub status_details: StatusDetails,
    pub boards: Vec<UserBoard>,
}

pub async fn get_boards(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
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

    // this will get all the BoardMember objects that matches user's id
    let board_members: Vec<BoardMember> =
        match BoardMember::get_board_members_by_user_id(&pool, user.id).await {
            Err(e) => {
                log::error!("{}", e);
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error, try again later",
                );
            }
            Ok(None) => Vec::new(),
            Ok(Some(bms)) => bms,
        };

    let mut boards: Vec<UserBoard> = Vec::new();

    if board_members.len() > 0 {
        for bm in board_members {
            match Board::get_by_id(&pool, bm.board_id).await {
                Err(e) => {
                    log::error!("{}", e);
                    continue;
                }
                Ok(None) => continue,
                Ok(Some(b)) => {
                    let board_url = match req.url_for("get_board", &[&b.name]) {
                        Ok(bu) => bu.to_string().clone(),
                        Err(e) => {
                            log::error!("{}", e);

                            format!("/boards/{}", &b.name)
                        }
                    };
                    boards.push(UserBoard {
                        name: b.name,
                        url: board_url,
                    });
                }
            }
        }
    }
    let code = StatusCode::OK;
    let status_details = StatusDetails::new(&code);
    let request_details = RequestDetails::new(&req);

    let mut response_builder = HttpResponse::build(code);
    response_builder.content_type("application/json");
    return response_builder.json(GetBoardsResponse {
        request_details: request_details,
        status_details: status_details,
        boards: boards,
    });
}
