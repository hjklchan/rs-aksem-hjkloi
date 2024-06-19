use axum::{extract::State, Json};

use crate::{
    app_state::AppState,
    result::{error::db::DatabaseError, OhMyResult},
};

#[derive(serde::Deserialize, Debug)]
pub struct CreateTicketReq {
    assignee_id: Option<u64>,
    title: String,
    description: Option<String>,
    body: Option<String>,
    status: u8,
}

#[derive(serde::Serialize)]
pub struct CreateTicketRep {
    new_id: u64,
}

pub async fn create_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateTicketReq>,
) -> OhMyResult<Json<CreateTicketRep>> {
    let CreateTicketReq {
        assignee_id,
        title,
        description,
        body,
        status,
    } = req;
    let sql = r#"
        INSERT INTO `tickets` ( `assignee_id`, `title`, `description`, `body`, `status`, `created_at`, `updated_at` )
        VALUES ( ?, ?, ?, ?, ?, NOW(), NOW() );
    "#;
    let new_id = sqlx::query(sql)
        .bind(assignee_id)
        .bind(title)
        .bind(description)
        .bind(body)
        .bind(status)
        .execute(&state.db)
        .await
        .map(|result| result.last_insert_id())
        .map_err(|err| DatabaseError(err))?;

    Ok(Json(CreateTicketRep { new_id }))
}
