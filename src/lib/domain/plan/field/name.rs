use crate::domain::PlanError::InvalidName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, InvalidName> {
        if name.trim().is_empty() {
            Err(InvalidName("Name is empty".to_owned()))
        } else {
            Ok(Self(name.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
