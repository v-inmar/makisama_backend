use std::{str::FromStr, time::Duration};

use sqlx::{
    MySqlPool,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
};

pub struct DatabasePool {
    pub pool: MySqlPool,
}

impl DatabasePool {
    pub async fn new(url: &str) -> sqlx::Result<DatabasePool> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60 * 5))
            .connect_with(MySqlConnectOptions::from_str(url)?)
            .await?;

        Ok(DatabasePool { pool: pool })
    }

    pub async fn ping(&self) -> sqlx::Result<String> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .expect("nope");

        Ok(format!("pong!"))
    }
}
