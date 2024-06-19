use axum::{http::StatusCode, response::IntoResponse, Json};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct DatabaseError(pub sqlx::error::Error);

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for DatabaseError {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        // Return the response directly!
        let json_tmpl = serde_json::json!({"code": 10000, "message": self.to_string()});

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            (Json(json_tmpl).into_response()).into_response(),
        )
            .into_response()
    }
}
