use crate::domain::PlanError::InvalidName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeID(String);

impl EmployeeID {
    pub fn new(employee_id: &str) -> Result<Self, InvalidName> {
        if employee_id.trim().is_empty() {
            Err(InvalidName("employee_id is empty".to_owned()))
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
