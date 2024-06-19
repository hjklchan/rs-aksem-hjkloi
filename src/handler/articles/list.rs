use axum::{extract::State, Json};

use crate::{
    app_state::AppState,
    result::{error::db::DatabaseError, OhMyResult},
};

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ListItem {
    id: u64,
    title: String,
    description: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(serde::Serialize)]
pub struct ListRep {
    items: Vec<ListItem>,
}

pub async fn list_handler(
    State(AppState { ref db }): State<AppState>,
) -> OhMyResult<Json<ListRep>> {
    Ok(Json(ListRep { items: sqlx::query_as("SELECT `id`, `title`, `description` AS `description`, `created_at` AS `created_at` FROM `articles` WHERE `status` = 0")
        .fetch_all(db)
        .await
        .map_err(|err| DatabaseError(err))?}))
}
