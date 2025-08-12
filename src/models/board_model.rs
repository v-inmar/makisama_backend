use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Board {
    pub id: u64,
    pub datetime_created: DateTime<Utc>,
    pub datetime_deleted: Option<DateTime<Utc>>,
    pub pid_id: u64,
    pub name_id: u64,
    pub description_id: Option<u64>,
}

/*

CREATE TABLE board (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    datetime_deleted DATETIME NULL,
    pid_id BIGINT UNSIGNED NOT NULL UNIQUE,
    name_id BIGINT UNSIGNED NOT NULL,
    description_id BIGINT UNSIGNED NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (name_id) REFERENCES board_name(id),
    FOREIGN KEY (pid_id) REFERENCES board_pid(id),
    FOREIGN KEY (description_id) REFERENCES board_description(id)
);

*/
