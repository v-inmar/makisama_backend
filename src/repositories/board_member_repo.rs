use crate::models::board_member_model::BoardMember;
use sqlx::{MySql, Transaction};

impl BoardMember {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        board_id: i64,
        user_id: i64,
        is_owner: bool,
        is_admin: bool,
    ) -> Result<BoardMember, Box<dyn std::error::Error>> {
        // Insert the new board member into the database
        sqlx::query!(
            r#"
            INSERT INTO board_member (board_id, user_id, is_owner, is_admin)
            VALUES (?, ?, ?, ?)
            "#,
            board_id,
            user_id,
            is_owner,
            is_admin
        )
        .execute(&mut **tx)
        .await?;

        // Fetch the newly inserted board member using LAST_INSERT_ID()
        let row = sqlx::query!(
            r#"
            SELECT id, datetime_created, board_id, user_id, is_owner, is_admin
            FROM board_member
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        // Manually convert i8 (TINYINT(1)) to bool (0 -> false, 1 -> true)
        let board_member = BoardMember {
            id: row.id,
            datetime_created: row.datetime_created,
            board_id: row.board_id,
            user_id: row.user_id,
            is_owner: row.is_owner != 0, // Convert i8 to bool
            is_admin: row.is_admin != 0, // Convert i8 to bool
        };

        // Return the BoardMember object
        Ok(board_member)
    }
}
