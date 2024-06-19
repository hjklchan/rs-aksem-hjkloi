use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    result::{error::db::DatabaseError, OhMyResult},
};

#[derive(Deserialize)]
pub struct ChangeStatusReq {
    new_status: u8,
}

pub async fn change_status_handler(
    State(state): State<AppState>,
    Path(ticket_id): Path<u64>,
    Json(req): Json<ChangeStatusReq>,
) -> OhMyResult<()> {
    let sql = "UPDATE `tickets` SET `status` = ? WHERE `id` = ? LIMIT 1";
    let _result = sqlx::query(sql)
        .bind(req.new_status)
        .bind(ticket_id)
        .execute(&state.db)
        .await
        .map(|result| {
            if result.rows_affected() == 0 {
                // ? DO SOMETHING HERE
            }
        })
        .map_err(|err| DatabaseError(err))?;
    Ok(())
}
