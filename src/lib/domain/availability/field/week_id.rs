use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekId(String);

impl WeekId {
    pub fn new() -> Self {
        let time = Utc::now();
        let year = time.year();
        let week = time.iso_week().week();
        let week_id = format!("{}-{}", year, week);
        let week_id = format!("{}-{}", year, week);
        Self(week_id)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
