use sqlx::mysql::MySqlPool;

/// Axum 全局状态数据结构
#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

pub fn new(db: MySqlPool) -> AppState {
    AppState { db }
}
