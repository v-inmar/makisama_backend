use crate::models::board_member_model::BoardMember;
use sqlx::{MySql, Pool, Transaction};

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

    pub async fn get_board_members_by_user_id_and_board_id(
        pool: &Pool<MySql>,
        user_id: i64,
        board_id: i64,
    ) -> Result<Option<BoardMember>, sqlx::error::Error> {
        let bm = sqlx::query!(
            r#"
            SELECT id, datetime_created, board_id, user_id, is_owner, is_admin
            FROM board_member
            WHERE board_id=? AND user_id=?
            "#,
            board_id,
            user_id
        )
        .fetch_optional(pool)
        .await?
        .map(|row| BoardMember {
            // needed to convert the is_owner and is_admin from i8 (mysql) to bool
            id: row.id,
            datetime_created: row.datetime_created,
            board_id: row.board_id,
            user_id: row.user_id,
            is_owner: row.is_owner != 0,
            is_admin: row.is_admin != 0,
        });

        Ok(bm)
    }

    pub async fn get_board_members_by_user_id(
        pool: &Pool<MySql>,
        user_id: i64,
    ) -> Result<Option<Vec<BoardMember>>, sqlx::error::Error> {
        let bms = sqlx::query!(
            r#"
        SELECT id, datetime_created, board_id, user_id, is_owner, is_admin
        FROM board_member
        WHERE user_id=?
        "#,
            user_id
        )
        .fetch_all(pool)
        .await?
        .into_iter() // Change to .into_iter() for proper ownership transfer
        .map(|row| BoardMember {
            id: row.id,
            datetime_created: row.datetime_created,
            board_id: row.board_id,
            user_id: row.user_id,
            // Convert i8 to bool
            is_owner: row.is_owner != 0,
            is_admin: row.is_admin != 0,
        })
        .collect(); // Collect into a vector

        Ok(Some(bms)) // Return wrapped in Some (as the function returns Option<Vec<_>>)
    }
}
