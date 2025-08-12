use sqlx::{MySql, Pool, Transaction};

use crate::models::board_model::Board;

impl Board {
    pub async fn new(
        tx: &mut Transaction<'_, MySql>,
        name_id: i64,
    ) -> Result<Board, Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO board (
            name_id
            )
            VALUES (
            ?
            )
            "#,
            name_id
        )
        .execute(&mut **tx)
        .await?;

        let row: Board = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, deleted, name_id
            FROM board
            WHERE id = LAST_INSERT_ID()
            "#
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(row)
    }

    pub async fn get_by_name_id(
        pool: &Pool<MySql>,
        name_id: i64,
    ) -> Result<Option<Board>, sqlx::error::Error> {
        let board = sqlx::query_as!(
            Board,
            r#"
            SELECT id, datetime_created, deleted, name_id
            FROM board
            WHERE name_id = ?
            AND deleted = NULL
            "#,
            name_id
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
            SELECT id, datetime_created, deleted, name_id
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
