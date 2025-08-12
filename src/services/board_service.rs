use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool, Transaction};

use crate::models::{
    board_member_model::BoardMember, board_model::Board, board_name_model::BoardName,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBoard {
    pub name: String,
    pub url: String,
    pub is_owner: bool,
    pub is_admin: bool,
}

pub async fn create_board_service(
    pool: &Pool<MySql>,
    name: &str,
    user_id: i64,
) -> Result<Board, Box<dyn std::error::Error>> {
    let mut tx: Transaction<'_, MySql> = pool.begin().await?;

    // deal with board name
    let board_name = match BoardName::get_by_name(&pool, &name.to_lowercase()).await? {
        Some(bn) => {
            // A board name with this name exists. Check if it's in use.
            if Board::get_by_name_id(&pool, bn.id).await?.is_some() {
                return Err("Board name already in use".into());
            }
            bn
        }
        None => {
            // The board name does not exist. Create a new one.
            let bn = BoardName::new(&mut tx, &name.to_lowercase()).await?;
            bn
        }
    };

    // deal with board
    let board = Board::new(&mut tx, board_name.id).await?;
    let _ = BoardMember::new(&mut tx, board.id, user_id, true, true).await?;

    tx.commit().await?;

    Ok(board)
}

pub async fn get_userboards_by_userid_service(
    pool: &Pool<MySql>,
    user_id: i64,
    page: i64,
    per_page: i64,
) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
    let mut boards: Vec<Board> = Vec::new();

    if let Some(board_members) =
        BoardMember::get_board_members_by_user_id(&pool, user_id, page, per_page).await?
    {
        // let x: Vec<UserBoard> = board_members
        //     .iter()
        //     .map(async move |&bm| {
        //         let b = Board::get_by_id(&pool, bm.board_id)
        //             .await
        //             .is_ok_and(|t| t.is_some());
        //         UserBoard {

        //         }
        //     })
        //     .collect();

        if board_members.len() > 0 {
            for board_member in board_members {
                match Board::get_by_id(&pool, board_member.board_id).await {
                    Err(e) => {
                        log::error!("{}", e);
                        continue;
                    }
                    Ok(None) => {
                        log::error!(
                            "Board::get_by_id for id: {} returns None",
                            board_member.board_id
                        );
                        continue;
                    }
                    Ok(Some(board)) => {
                        if board.deleted.is_some() {
                            continue;
                        } else {
                            if board.name_id.is_some() {
                                let url = match BoardName::get_by_id(&pool, board.name_id).await {
                                    Err(e) => {
                                        log::error!("{}", e);
                                        continue;
                                    }
                                    Ok(None) => continue,
                                    Ok(Some(n)) => {}
                                };
                            } else {
                                continue;
                            }
                            // boards.push(board);
                            // if let Some(name_id) = board.name_id {
                            //     match BoardName::get_by_id(&pool, name_id).await {
                            //         Err(e) => {
                            //             log::error!("{}", e);
                            //             continue;
                            //         }
                            //         Ok(None) => {
                            //             log::error!(
                            //                 "BoardName::get_by_id for id: {} returns None",
                            //                 name_id
                            //             );

                            //             continue;
                            //         }
                            //         Ok(Some(board_name)) => {
                            //             let board_url =
                            //                 match req.url_for("get_board", &[&board_name.name]) {
                            //                     Err(e) => {
                            //                         log::error!("{}", e);
                            //                         format!("/boards/{}", &board_name.name)
                            //                     }
                            //                     Ok(url) => url.to_string().clone(),
                            //                 };

                            //             boards.push(UserBoard {
                            //                 name: board_name.name,
                            //                 url: board_url,
                            //                 is_owner: board_member.is_owner,
                            //                 is_admin: board_member.is_admin,
                            //             });
                            //         }
                            //     }
                            // }
                        }
                    }
                }
            }
        }
    }

    Ok(boards)
}
