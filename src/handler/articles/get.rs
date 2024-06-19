use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    result::{error::db::DatabaseError, OhMyResult},
};

#[derive(Debug, serde::Serialize)]
pub struct GetRep {
    id: u64,
    title: String,
    body: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Debug, serde::Serialize)]
pub struct Item {
    id: u64,
    title: String,
    markdown_body: Option<String>,
    markdown_file: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_handler(
    State(AppState { ref db }): State<AppState>,
    Path(article_id): Path<u64>,
) -> OhMyResult<Json<GetRep>> {
    let json_rep = sqlx::query!("SELECT `id`, `title` AS `title`, `markdown_body` AS `markdown_body?`, `markdown_file` AS `markdown_file?`, `created_at` AS `created_at?` FROM `articles` WHERE `id` = ? LIMIT 1", article_id)
        .fetch_one(db)
        .await
        .map(|rec| {
            let body = rec.markdown_body.and_then(|ref body| {
                // TODO: Here needs to be optimized.
                if body.len() == 0 {
                    rec.markdown_file
                } else {
                    Some(body.to_owned())
                }
            }).and_then(|ref file| {
                // If type is file, And then request that
                if file.len() == 0 {
                    None
                } else {
                    // TODO: Request and parse markdown text
                    Some(file.to_owned())
                }
            });
            Json(GetRep {
                id: rec.id,
                title: rec.title,
                body: body.unwrap_or("No content...".into()),
                created_at: rec.created_at,
            })
        })
        .map_err(|err| DatabaseError(err))?;

    Ok(json_rep)
}
