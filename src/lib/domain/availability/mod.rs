pub mod field;
use crate::data::DbId;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AvailabilityError {
    #[error("Id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Invalid day: {0}")]
    InvalidDay(String),
    #[error("Invalid time: {0}")]
    InvalidTime(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct AvailabilitySpot {
    //Todo needs to add id
    // pub id: field::Id,
    pub user_id: field::AvailabilityId,
    pub weekly_id: field::WeeklyId,
    pub name: field::Name,
    pub day: field::Days,
    pub time: field::Time,
}
impl AvailabilitySpot {
    pub fn create(id: &str, name: &str, day: &str, time: &str) -> Self {
        Self {
            // id: field::id::Id::new()
            user_id: field::AvailabilityId::new(DbId::from_str(id).unwrap()),
            weekly_id: field::WeeklyId::new(),
            name: field::Name::new(name).unwrap(),
            day: field::Days::new(day),
            time: field::Time::new(time),
        }
    }
    pub fn new(id: &str, name: &str, day: &str, time: &str) -> Self {
        Self {
            // id: field::id::Id::new(),
            user_id: field::AvailabilityId::new(DbId::from_str(id).unwrap()),
            weekly_id: field::WeeklyId::new(),
            name: field::Name::new(name).unwrap(),
            day: field::Days::new(day),
            time: field::Time::new(time),
        }
    }
}
