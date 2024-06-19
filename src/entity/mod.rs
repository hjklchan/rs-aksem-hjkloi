use chrono::{self, Utc};

/// Ticket database entity
#[derive(Debug, sqlx::FromRow)]
pub struct Ticket {
    pub id: u64,
    pub assignee_id: Option<u64>,
    pub title: String,
    pub description: String,
    pub body: String,
    pub status: u8,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}
