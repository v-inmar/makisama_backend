use sqlx::{MySql, Pool, Transaction};

use crate::models::{board_member_model::BoardMember, board_model::Board};

pub async fn add_new_board_service(
    pool: &Pool<MySql>,
    name: &str,
    user_id: i64,
) -> Result<Board, Box<dyn std::error::Error>> {
    let mut tx: Transaction<'_, MySql> = pool.begin().await?;

    let board = Board::new(&mut tx, &name).await?;
    let _ = BoardMember::new(&mut tx, board.id, user_id, true, true).await?;

    tx.commit().await?;

    Ok(board)
}
