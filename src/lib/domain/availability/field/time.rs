use serde::{Deserialize, Serialize};

use crate::domain::ScheduleTime;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Time(ScheduleTime);

impl Time {
    pub fn new(time: &str) -> Self {
        Self(ScheduleTime::from_str(time))
    }
    pub fn into_inner(self) -> ScheduleTime {
        self.0
    }
    pub fn create(time: ScheduleTime) -> Self {
        Self(time)
    }
    pub fn as_str(&self) -> String {
        ScheduleTime::from_const(&self.0)
    }
}
