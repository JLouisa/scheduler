use serde::{Deserialize, Serialize};

use crate::domain::ScheduleDay;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Days(ScheduleDay);

impl Days {
    pub fn new(days: &str) -> Self {
        Self(ScheduleDay::from_str(days))
    }
    pub fn into_inner(&self) -> &ScheduleDay {
        &self.0
    }
    pub fn create(days: ScheduleDay) -> Self {
        Self(days)
    }
    pub fn as_str(&self) -> String {
        ScheduleDay::from_const(&self.0)
    }
}
