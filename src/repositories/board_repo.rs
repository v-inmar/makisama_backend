use sqlx::{MySql, Pool, Transaction};

use crate::models::board_model::Board;

impl Board {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        name: &str,
    ) -> Result<Board, Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO board (
            name
            )
            VALUES (
            ?
            )
            "#,
            name
        )
        .execute(&mut **tx)
        .await?;

        let board: Board = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, name
            FROM board
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(board)
    }

    pub async fn get_by_name(
        pool: &Pool<MySql>,
        name: &str,
    ) -> Result<Option<Board>, sqlx::error::Error> {
        let board = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, name
            FROM board
            WHERE name = ?
            "#,
            name
        )
        .fetch_optional(pool)
        .await?;

        Ok(board)
    }

    pub async fn get_by_id(
        pool: &Pool<MySql>,
        id: i64,
    ) -> Result<Option<Board>, sqlx::error::Error> {
        let board = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, name
            FROM board
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(board)
    }
}
