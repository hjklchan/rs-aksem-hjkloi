use axum::{extract::State, Json};

use crate::{
    app_state::AppState,
    result::{error::db::DatabaseError, OhMyResult},
};

use super::Ticket;

#[derive(Debug, serde::Serialize)]
pub struct GetTicketListRep {
    items: Vec<Ticket>,
}

pub async fn list_handler(State(state): State<AppState>) -> OhMyResult<Json<GetTicketListRep>> {
    Ok(sqlx::query_as!(
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
        "#
    )
    .fetch_all(&state.db)
    .await
    .map(|records| Json(GetTicketListRep { items: records }))
    .map_err(|err| DatabaseError(err))?)
}
