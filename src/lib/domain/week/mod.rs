use crate::data;
pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeekError {
    #[error("Id parse error: {0}")]
    IdError(#[from] uuid::Error),
    #[error("Invalid name: {0}")]
    InvalidNameError(String),
    #[error("Invalid day: {0}")]
    InvalidDayError(String),
    #[error("Invalid time: {0}")]
    InvalidTimeError(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Week {
    pub week_id: data::DbId,
    pub weekly_id: field::WeeklyId,
    pub name: field::Name,
    pub monday: field::Days,
    pub tuesday: field::Days,
    pub wednesday: field::Days,
    pub thursday: field::Days,
    pub friday: field::Days,
    pub saturday: field::Days,
    pub sunday: field::Days,
}
