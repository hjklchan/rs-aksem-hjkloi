use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    result::{
        error::{db::DatabaseError, ticket::TicketError, ApiError},
        OhMyResult,
    },
};

use super::Ticket;

#[derive(Debug, serde::Serialize)]
pub struct GetTicketRep {
    item: Ticket,
}

pub async fn get_handler(
    State(state): State<AppState>,
    Path(ticket_id): Path<u64>,
) -> OhMyResult<Json<GetTicketRep>> {
    let ticket = sqlx::query_as!(
        Ticket,
        r#"
        SELECT
            `id`,
            `assignee_id` as `assignee_id?`,
            `title` as `title`,
            `description` as `description?`,
            `body` as `body?`,
            `status`,
            `created_at` as `created_at?`,
            `updated_at` as `updated_at?`
        FROM `tickets`
        WHERE `id` = ? LIMIT 1
        "#,
        ticket_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            return ApiError::Ticket(TicketError::NotFound);
        }
        other_err => {
            return ApiError::Database(DatabaseError(other_err));
        }
    })?;

    Ok(Json(GetTicketRep { item: ticket }))
}
