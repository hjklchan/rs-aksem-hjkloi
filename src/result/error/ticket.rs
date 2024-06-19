use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug, thiserror::Error)]
pub enum TicketError {
    #[error("Ticket not found")]
    NotFound,
}

impl IntoResponse for TicketError {
    fn into_response(self) -> axum::response::Response {
        let (err_code, status_code) = match self {
            Self::NotFound => (20001, StatusCode::NOT_FOUND),
        };

        let json_tmpl = serde_json::json!({"code": err_code, "message": self.to_string()});
        (status_code, Json(json_tmpl).into_response()).into_response()
    }
}
