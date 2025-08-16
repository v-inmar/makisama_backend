// use crate::models::user_lastname_model::Lastname;
// use sqlx::{MySql, Pool, Transaction};

// impl Lastname {
//     pub async fn new(
//         tx: &mut Transaction<'_, MySql>,
//         value: &str,
//     ) -> Result<Lastname, sqlx::error::Error> {
//         // insert row
//         sqlx::query!(
//             r#"
//             INSERT INTO lastname (value)
//             VALUES (?)
//             "#,
//             value
//         )
//         .execute(&mut **tx)
//         .await?;

//         // get last inserted row using last insert id
//         let lastname = sqlx::query_as!(
//             Lastname,
//             r#"
//             SELECT id, value, datetime_created
//             FROM lastname
//             WHERE id = LAST_INSERT_ID()
//             "#,
//         )
//         .fetch_one(&mut **tx)
//         .await?;

//         Ok(lastname)
//     }

//     pub async fn get_by_value(
//         pool: &Pool<MySql>,
//         value: &str,
//     ) -> Result<Option<Lastname>, sqlx::error::Error> {
//         let lastname = sqlx::query_as!(
//             Lastname,
//             r#"
//             SELECT id, value, datetime_created
//             FROM lastname
//             WHERE value = ?
//             "#,
//             value
//         )
//         .fetch_optional(pool)
//         .await?;

//         Ok(lastname)
//     }
// }
