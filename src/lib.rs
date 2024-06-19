pub mod app_state;
pub mod config;
pub mod handler;
pub mod entity;
pub mod result;

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

/// ### 连接数据库连接池
///
/// - 当连接发生错误时会直接 panic,  
/// - 否则可以调用 connect_pool 返回 Result<Pool<MySql>, sqlx::Error>
pub async fn must_connect_pool(database_url: impl AsRef<str>) -> Pool<MySql> {
    MySqlPoolOptions::new()
        .connect(database_url.as_ref())
        .await
        .unwrap()
}

/// ### 连接数据库连接池
///
/// 返回 Result<Pool<MySql>, sqlx::Error>
pub async fn connect_pool(database_url: impl AsRef<str>) -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new().connect(database_url.as_ref()).await
}
