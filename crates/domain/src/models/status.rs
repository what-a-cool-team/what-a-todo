use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum Status {
    Created,
    InProgress,
    Completed,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "created" => Ok(Status::Created),
            "inprogress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            _ => Err(()),
        }
    }
}