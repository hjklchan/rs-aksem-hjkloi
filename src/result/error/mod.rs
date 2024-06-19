// errors
pub mod db;
pub mod ticket;

use axum::response::IntoResponse;
use thiserror::Error;

/// ### 此处定义错误集
#[derive(Debug, Error)]
pub enum ApiError {
    // 数据库相关的错误
    #[error(transparent)]
    Database(#[from] db::DatabaseError),
    // TODO: 模块级的错误示例
    #[error("{0}")]
    Ticket(#[from] ticket::TicketError),
}

/// ### 为错误集实现 IntoResponse 特征
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(err) => err.into_response(),
            Self::Ticket(err) => err.into_response(),
        }
    }
}


