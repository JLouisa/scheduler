use crate::domain::UserError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MaxDays(u8);

impl MaxDays {
    pub fn new(max_days: &str) -> Result<Self, UserError> {
        if max_days.trim().is_empty() {
            Err(InvalidName("Min days is empty".to_owned()))
        } else {
            Ok(Self(max_days.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
