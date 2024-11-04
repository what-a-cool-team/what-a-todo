use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,

    // Avoid exposing it in the API
    #[serde(skip_serializing)]
    pub password_hash: String,
}

impl User {
    pub fn new(id: String, password_hash: String) -> Self {
        Self { id, password_hash }
    }
}