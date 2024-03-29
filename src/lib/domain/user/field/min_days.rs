use crate::domain::user::UserError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MinDays(u8);

impl MinDays {
    pub fn new(min_days: &str) -> Result<Self, UserError> {
        if min_days.trim().is_empty() {
            Err(UserError::InvalidMinDays("Min days is empty".to_owned()))
        } else {
            match min_days.parse::<u8>() {
                Ok(number) => Ok(Self(number)),
                Err(_) => Err(UserError::InvalidMinDays(
                    "Invalid, not a number".to_owned(),
                )),
            }
        }
    }

    pub fn into_inner(self) -> u8 {
        self.0
    }
}
