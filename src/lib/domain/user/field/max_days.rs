use crate::domain::user::UserError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxDays(u8);

impl MaxDays {
    pub fn new(max_days: &str) -> Result<Self, UserError> {
        if max_days.trim().is_empty() {
            Err(UserError::InvalidMinDays("Max days is empty".to_owned()))
        } else {
            match max_days.parse::<u8>() {
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
