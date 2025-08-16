// use sqlx::{MySql, Pool, Transaction};

// use crate::models::board_name_model::BoardName;

// impl BoardName {
//     pub async fn new(
//         tx: &mut Transaction<'_, MySql>,
//         name: &str,
//     ) -> Result<BoardName, sqlx::error::Error> {
//         sqlx::query!(
//             r#"
//             INSERT INTO board_name (name)
//             VALUES (?)
//             "#,
//             name
//         )
//         .execute(&mut **tx)
//         .await?;

//         let row = sqlx::query_as!(
//             BoardName,
//             r#"
//             SELECT id, datetime_created, name
//             FROM board_name
//             WHERE id = LAST_INSERT_ID()
//             "#
//         )
//         .fetch_one(&mut **tx)
//         .await?;

//         Ok(row)
//     }

//     pub async fn get_by_id(
//         pool: &Pool<MySql>,
//         id: i64,
//     ) -> Result<Option<BoardName>, sqlx::error::Error> {
//         let row = sqlx::query_as!(
//             BoardName,
//             r#"
//             SELECT id, datetime_created, name
//             FROM board_name
//             WHERE id = ?
//             "#,
//             id
//         )
//         .fetch_optional(pool)
//         .await?;

//         Ok(row)
//     }

//     pub async fn get_by_name(
//         pool: &Pool<MySql>,
//         name: &str,
//     ) -> Result<Option<BoardName>, sqlx::error::Error> {
//         let row = sqlx::query_as!(
//             BoardName,
//             r#"
//             SELECT id, datetime_created, name
//             FROM board_name
//             WHERE name = ?
//             "#,
//             name
//         )
//         .fetch_optional(pool)
//         .await?;

//         Ok(row)
//     }
// }
