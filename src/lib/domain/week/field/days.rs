use crate::domain::{week::WeekError, ScheduleDay};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Days(String);

impl Days {
    pub fn new(day: &str) -> Result<Self, WeekError> {
        if day.trim().is_empty() {
            Err(WeekError::InvalidDayError("Day is empty".to_owned()))
        } else {
            Ok(Self(day.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn to_const(self) -> ScheduleDay {
        ScheduleDay::from_str(self.0.as_str())
    }
}
