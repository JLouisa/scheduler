pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlanError {
    #[error("Id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Invalid day: {0}")]
    InvalidDay(String),
    #[error("Invalid time: {0}")]
    InvalidTime(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Plan {
    pub user_id: field::UserId,
    pub name: field::Name,
    pub day: field::Day,
    pub time: field::Time,
}
