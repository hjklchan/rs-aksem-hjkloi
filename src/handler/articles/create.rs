use axum::{extract::State, Json};

use crate::{
    app_state::AppState,
    result::{error::db::DatabaseError, OhMyResult},
};

#[derive(Debug, serde::Deserialize)]
pub struct CreateReq {
    title: String,
    description: Option<String>,
    markdown_body: Option<String>,
    markdown_file: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateRep {
    new_id: u64,
}

pub async fn create_handler(
    State(AppState { ref db }): State<AppState>,
    Json(CreateReq {
        title,
        description,
        mut markdown_body,
        mut markdown_file,
    }): Json<CreateReq>,
) -> OhMyResult<Json<CreateRep>> {
    let sql = r#"
    INSERT INTO `articles` ( `title`, `description`, `markdown_body`, `markdown_file`, `status`, `created_at`, `updated_at` )
    VALUES ( ?, ?, ?, ?, 0, NOW(), NOW() )
    "#;

    if let Some(ref body) = markdown_body {
        if body.len() == 0 {
            markdown_body = None;
        } else {
            markdown_file = None;
        }
    }

    let new_id = sqlx::query(sql)
        .bind(title)
        .bind(description)
        .bind(markdown_body)
        .bind(markdown_file)
        .execute(db)
        .await
        .map(|res| res.last_insert_id())
        .map_err(|err| DatabaseError(err))?;

    Ok(Json(CreateRep { new_id }))
}
