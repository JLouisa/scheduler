use crate::domain::{week::WeekError, ScheduleTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Time(String);

impl Time {
    pub fn new(time: &str) -> Result<Self, WeekError> {
        if time.trim().is_empty() {
            Err(WeekError::InvalidTimeError("Time is empty".to_owned()))
        } else {
            Ok(Self(time.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_const(self) -> ScheduleTime {
        ScheduleTime::from_str(self.0.as_str())
    }
}
