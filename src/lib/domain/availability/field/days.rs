use serde::{Deserialize, Serialize};

use crate::domain::ScheduleDay;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Days(String);

impl Days {
    pub fn new(days: &str) -> Option<Self> {
        if days.trim().is_empty() {
            None
        } else {
            Some(Self(days.to_owned()))
        }
    }
    pub fn into_inner(self) -> ScheduleDay {
        ScheduleDay::from_str(self.0.as_str())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
