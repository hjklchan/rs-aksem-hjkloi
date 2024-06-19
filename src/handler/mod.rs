pub mod root;
pub mod tickets;
pub mod articles;

use axum::{
    routing::{delete, get, patch, post}, Router
};
use root::root_handler;

use crate::app_state::AppState;

/// 路由
pub fn routes(state: AppState) -> Router
{
    Router::new()
        .route("/", get(root_handler))
        // Ticket 模块
        .route("/tickets", post(tickets::create_handler))
        .route("/tickets/:id", delete(tickets::delete_handler))
        .route("/tickets/:id", patch(tickets::update_handler))
        .route("/tickets/:id", get(tickets::get_handler))
        .route("/tickets", get(tickets::list_handler))
        .route("/tickets/:id/status", patch(tickets::change_status_handler))
        // Article 模块
        .route("/articles", get(articles::list_handler))
        .route("/articles", post(articles::create_handler))
        .route("/articles/:article_id", get(articles::get_handler))
        .with_state(state)
}
