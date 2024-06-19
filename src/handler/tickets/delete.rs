use axum::extract::{Path, State};

use crate::{
    app_state::AppState,
    result::{
        error::{db::DatabaseError, ticket::TicketError},
        OhMyResult,
    },
};

pub async fn delete_handler(
    State(state): State<AppState>,
    Path(ticket_id): Path<u64>,
) -> OhMyResult<()> {
    sqlx::query("DELETE FROM `tickets` WHERE `id` = ?")
        .bind(ticket_id)
        .execute(&state.db)
        .await
        .map(|result| {
            if result.rows_affected() == 0 {
                Err(TicketError::NotFound.into())
            } else {
                Ok(())
            }
        })
        .map_err(|err| DatabaseError(err))?
}
