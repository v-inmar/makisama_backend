use actix_web::HttpResponse;
use actix_web::{HttpMessage, HttpRequest, Responder, http::StatusCode, web};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::board_model::Board;
use crate::models::board_name_model::BoardName;
use crate::models::board_pid_model::BoardPid;
use crate::models::board_user_model::BoardUser;
use crate::utils::json_response_utils::{RequestDetails, StatusDetails};
use crate::utils::{
    handler_utils::get_user_by_auth_identity, json_response_utils::JsonGeneralResponse,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBoardsResponseData {
    pub name: String,
    pub url: String,
    pub is_owner: bool,
    pub is_admin: bool,
    pub pid: String,
    pub datetime_created: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBoardsResponse {
    pub request_details: RequestDetails,
    pub status_details: StatusDetails,
    pub boards: Vec<GetBoardsResponseData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBoardsQueryParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

pub async fn get_boards(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    query: web::Query<GetBoardsQueryParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(1) as u64;
    let per_page = query.per_page.unwrap_or(10) as u64;

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
    // let board_members: Vec<BoardMember> =
    //     match BoardMember::get_board_members_by_user_id(&pool, user.id, page, per_page).await {
    //         Err(e) => {
    //             log::error!("{}", e);
    //             return JsonGeneralResponse::make_response(
    //                 &req,
    //                 &StatusCode::INTERNAL_SERVER_ERROR,
    //                 "Server error, try again later",
    //             );
    //         }
    //         Ok(None) => Vec::new(),
    //         Ok(Some(bms)) => bms,
    //     };

    let mut boards: Vec<GetBoardsResponseData> = Vec::new();

    match BoardUser::get_by_user_id(&pool, user.id, page, per_page).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later.",
            );
        }
        Ok(board_users) => {
            for bu in board_users {
                // let mut gbrd = GetBoardsResponseData {
                //     is_admin: bu.is_admin,
                //     is_owner: bu.is_owner,
                //     name: String::new(),
                //     url: String::new(),
                //     pid: String::new(),
                //     datetime_created: DateTime::<Utc>::from_timestamp(1633024800, 0),
                // };

                // Get board data
                match Board::get_by_id(&pool, bu.board_id).await {
                    Err(e) => {
                        log::error!("{}", e);
                        continue;
                    }
                    Ok(None) => {
                        log::error!("Board not found for board_id: {}", bu.board_id);
                        continue;
                    }
                    Ok(Some(b)) => {
                        let name;
                        let pid;
                        let url;
                        // Name handling
                        if let Ok(Some(bn)) = BoardName::get_by_id(&pool, b.name_id).await {
                            name = bn.value;
                        } else {
                            log::error!("Board name not found for board_id: {}", bu.board_id);
                            continue;
                        }

                        // PID handling and URL generation
                        if let Ok(Some(bp)) = BoardPid::get_by_id(&pool, b.pid_id).await {
                            pid = bp.value.clone();
                            url = req.url_for("get_board", &[&bp.value]).map_or_else(
                                |_| format!("/boards/{}", &bp.value).to_string(),
                                |url| url.to_string(),
                            );
                        } else {
                            log::error!("Board PID not found for board_id: {}", bu.board_id);
                            continue;
                        }

                        boards.push(GetBoardsResponseData {
                            is_admin: bu.is_admin,
                            is_owner: bu.is_owner,
                            name: name,
                            pid: pid,
                            url: url,
                            datetime_created: b.datetime_created,
                        });
                    }
                }
            }
        }
    }

    // let mut user_boards: Vec<UserBoard> = Vec::new();

    // match get_boards_by_userid_service(&pool, user.id, page, per_page).await {
    //     Err(e) => {
    //         log::error!("{}", e);
    //         return JsonGeneralResponse::make_response(&req, &StatusCode::INTERNAL_SERVER_ERROR, "Server error, try again later.");
    //     },
    //     Ok(boards) => {
    //         if boards.len() > 0 {
    //             boards.iter()
    //             .filter(|&&b| b.deleted.is_some())
    //             .filter(|&&b| b.name_id.is_some() )
    //             .filter(async move |&&b| BoardName::get_by_id(&pool, b.name_id).await.is_ok_and(|bn| bn.is_some()))
    //             .map(async move |&b| {
    //                 BoardName::get_by_id(&pool, b.name_id)
    //             });

    //             for board in boards {

    //                 if board.deleted.is_some(){
    //                     continue;
    //                 }else{

    //                     // get board name
    //                     if let Some(name_id) = board.name_id{
    //                         match BoardName::get_by_id(&pool, name_id).await{
    //                             Err(e) => {
    //                                 log::error!("{}", e);
    //                                 continue;
    //                             },
    //                             Ok(None) => {
    //                                 continue;
    //                             },
    //                             Ok(Some(board_name)) => {
    //                                 let board_url = match req.url_for("get_board", &[&board_name.name]) {

    //                                 }

    //                                 req.url_for(name, elements).
    //                             }
    //                         }
    //                     }

    //                 }
    //             }
    //         }
    //     }
    // }

    // if board_members.len() > 0 {
    //     for bm in board_members {
    //         match Board::get_by_id(&pool, bm.board_id).await {
    //             Err(e) => {
    //                 log::error!("{}", e);
    //                 continue;
    //             }
    //             Ok(None) => continue,
    //             Ok(Some(b)) => match BoardName::get_by_id(&pool, b.name_id).await {
    //                 Err(e) => {
    //                     log::error!("{}", e);
    //                     continue;
    //                 }
    //                 Ok(None) => {
    //                     continue;
    //                 }
    //                 Ok(Some(bn)) => {
    //                     let board_url = match req.url_for("get_board", &[&bn.name]) {
    //                         Err(e) => {
    //                             log::error!("{}", e);
    //                             format!("/boards/{}", &bn.name)
    //                         }
    //                         Ok(url) => url.to_string().clone(),
    //                     };

    //                     boards.push(UserBoard {
    //                         name: bn.name,
    //                         url: board_url,
    //                     });
    //                 }
    //             },
    //         }
    //     }
    // }
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
