use serde::{Deserialize, Serialize};

use crate::domain::ScheduleTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Time(String);

impl Time {
    pub fn new(time: &str) -> Option<Self> {
        if time.trim().is_empty() {
            None
        } else {
            Some(Self(time.to_owned()))
        }
    }
    pub fn into_inner(self) -> ScheduleTime {
        ScheduleTime::from_str(self.0.as_str())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
