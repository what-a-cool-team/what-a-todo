use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use super::status::Status;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(user_id: i32, name: String, description: Option<String>, status: Status, tags: Option<Vec<String>>) -> Self {
        Task {
            id: 0, // auto-generated
            user_id,
            name,
            description,
            status,
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}