use sqlx::{MySql, Pool, Transaction};

use crate::models::{
    board_member_model::BoardMember, board_model::Board, board_name_model::BoardName,
};

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
