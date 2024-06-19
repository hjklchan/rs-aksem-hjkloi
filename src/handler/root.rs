use axum::response::IntoResponse;

use crate::result::{error::ticket::TicketError, OhMyResult};

/// Test ok
pub async fn root_handler() -> impl IntoResponse {
    let rs: OhMyResult<()> = Err(TicketError::NotFound.into());

    rs
}
