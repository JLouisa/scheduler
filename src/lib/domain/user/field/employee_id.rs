use crate::domain::user::UserError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct EmployeeID(String);

impl EmployeeID {
    pub fn new(employee_id: &str) -> Result<Self, UserError> {
        if employee_id.trim().is_empty() {
            Err(UserError::InvalidEmployeeID(
                "employee_id is empty".to_owned(),
            ))
        } else {
            Ok(Self(employee_id.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
