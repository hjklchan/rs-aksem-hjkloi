mod change_status;
mod create;
mod delete;
mod get;
mod list;
mod update;

use ::chrono::Utc;
use sqlx::types::chrono::{self};

pub use change_status::change_status_handler;
pub use create::create_handler;
pub use delete::delete_handler;
pub use get::get_handler;
pub use list::list_handler;
pub use update::update_handler;

#[derive(Debug, serde::Serialize)]
pub struct Ticket {
    pub id: u64,
    pub assignee_id: Option<u64>, // ! Should related to User(Doesn't design it now)
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
    pub status: i8,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}
