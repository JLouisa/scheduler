use crate::domain::{user::UserError, Role};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RolePrimary(String);

impl RolePrimary {
    pub fn new(role: &str) -> Result<Self, UserError> {
        if role.trim().is_empty() {
            Err(UserError::InvalidRolePrimary(
                "Role Primary is empty".to_owned(),
            ))
        } else {
            Ok(Self(role.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn to_const(self) -> Role {
        Role::from_str(self.0.as_str())
    }
}
