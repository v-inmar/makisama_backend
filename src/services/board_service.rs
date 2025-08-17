use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::{
    handlers::board_handlers::add_board_handler::AddBoardRequestData,
    models::{
        board_description_model::BoardDescription, board_model::Board, board_name_model::BoardName,
        board_pid_model::BoardPid, board_user_model::BoardUser,
    },
};

pub async fn create_board(
    pool: &Pool<MySql>,
    user_id: u64,
    data: &AddBoardRequestData,
) -> Result<Board, Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;

    // board pid
    let board_pid: BoardPid;
    let mut counter_pid = 0;
    loop {
        if counter_pid == 6 {
            let err_msg = "Error while creating BoardPid. Try limit has been reached";
            log::error!("{}", err_msg);
            return Err(err_msg.into());
        }

        counter_pid += 1;
        let mut uuid_str = String::new();
        for _ in 0..4 {
            uuid_str.push_str(&Uuid::new_v4().to_string().replace('-', ""));
        }
        let pid_value = &uuid_str[0..128]; // just to make sure it will fit inside mysql column value
        match BoardPid::get_by_value(&pool, pid_value).await? {
            Some(_) => continue,
            None => {
                board_pid = BoardPid::new(&mut tx, pid_value).await?;
                break;
            }
        }
    }

    // board name
    let board_name = match BoardName::get_by_value(&pool, &data.name).await? {
        Some(bn) => bn,
        None => BoardName::new(&mut tx, &data.name).await?,
    };

    // board description
    let mut board_description: Option<BoardDescription> = None;

    if let Some(description) = &data.description {
        match BoardDescription::get_by_value(&pool, &description).await? {
            Some(bd) => {
                board_description = Some(bd);
            }
            None => {
                board_description = Some(BoardDescription::new(&mut tx, &description).await?);
            }
        }
    }

    // Extract the description_id (if it exists) from board_description
    let description_id = board_description.as_ref().map(|bd| bd.id);

    // board
    let board = Board::new(&mut tx, board_pid.id, board_name.id, description_id).await?;

    // board user
    BoardUser::new(&mut tx, board.id, user_id, true, true).await?;

    tx.commit().await?;

    Ok(board)
}

// use serde::{Deserialize, Serialize};
// use sqlx::{MySql, Pool, Transaction};

// use crate::models::{
//     board_member_model::BoardMember, board_model::Board, board_name_model::BoardName,
// };

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserBoard {
//     pub name: String,
//     pub url: String,
//     pub is_owner: bool,
//     pub is_admin: bool,
// }

// pub async fn create_board_service(
//     pool: &Pool<MySql>,
//     name: &str,
//     user_id: i64,
// ) -> Result<Board, Box<dyn std::error::Error>> {
//     let mut tx: Transaction<'_, MySql> = pool.begin().await?;

//     // deal with board name
//     let board_name = match BoardName::get_by_name(&pool, &name.to_lowercase()).await? {
//         Some(bn) => {
//             // A board name with this name exists. Check if it's in use.
//             if Board::get_by_name_id(&pool, bn.id).await?.is_some() {
//                 return Err("Board name already in use".into());
//             }
//             bn
//         }
//         None => {
//             // The board name does not exist. Create a new one.
//             let bn = BoardName::new(&mut tx, &name.to_lowercase()).await?;
//             bn
//         }
//     };

//     // deal with board
//     let board = Board::new(&mut tx, board_name.id).await?;
//     let _ = BoardMember::new(&mut tx, board.id, user_id, true, true).await?;

//     tx.commit().await?;

//     Ok(board)
// }

// pub async fn get_userboards_by_userid_service(
//     pool: &Pool<MySql>,
//     user_id: i64,
//     page: i64,
//     per_page: i64,
// ) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
//     let mut boards: Vec<Board> = Vec::new();

//     if let Some(board_members) =
//         BoardMember::get_board_members_by_user_id(&pool, user_id, page, per_page).await?
//     {
//         // let x: Vec<UserBoard> = board_members
//         //     .iter()
//         //     .map(async move |&bm| {
//         //         let b = Board::get_by_id(&pool, bm.board_id)
//         //             .await
//         //             .is_ok_and(|t| t.is_some());
//         //         UserBoard {

//         //         }
//         //     })
//         //     .collect();

//         if board_members.len() > 0 {
//             for board_member in board_members {
//                 match Board::get_by_id(&pool, board_member.board_id).await {
//                     Err(e) => {
//                         log::error!("{}", e);
//                         continue;
//                     }
//                     Ok(None) => {
//                         log::error!(
//                             "Board::get_by_id for id: {} returns None",
//                             board_member.board_id
//                         );
//                         continue;
//                     }
//                     Ok(Some(board)) => {
//                         if board.deleted.is_some() {
//                             continue;
//                         } else {
//                             if board.name_id.is_some() {
//                                 let url = match BoardName::get_by_id(&pool, board.name_id).await {
//                                     Err(e) => {
//                                         log::error!("{}", e);
//                                         continue;
//                                     }
//                                     Ok(None) => continue,
//                                     Ok(Some(n)) => {}
//                                 };
//                             } else {
//                                 continue;
//                             }
//                             // boards.push(board);
//                             // if let Some(name_id) = board.name_id {
//                             //     match BoardName::get_by_id(&pool, name_id).await {
//                             //         Err(e) => {
//                             //             log::error!("{}", e);
//                             //             continue;
//                             //         }
//                             //         Ok(None) => {
//                             //             log::error!(
//                             //                 "BoardName::get_by_id for id: {} returns None",
//                             //                 name_id
//                             //             );

//                             //             continue;
//                             //         }
//                             //         Ok(Some(board_name)) => {
//                             //             let board_url =
//                             //                 match req.url_for("get_board", &[&board_name.name]) {
//                             //                     Err(e) => {
//                             //                         log::error!("{}", e);
//                             //                         format!("/boards/{}", &board_name.name)
//                             //                     }
//                             //                     Ok(url) => url.to_string().clone(),
//                             //                 };

//                             //             boards.push(UserBoard {
//                             //                 name: board_name.name,
//                             //                 url: board_url,
//                             //                 is_owner: board_member.is_owner,
//                             //                 is_admin: board_member.is_admin,
//                             //             });
//                             //         }
//                             //     }
//                             // }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(boards)
// }
