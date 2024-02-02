use crate::domain::{user::UserError, Role};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RoleSecondary(Role);

impl RoleSecondary {
    pub fn new(role: &str) -> Result<RoleSecondary, UserError> {
        if role.trim().is_empty() {
            Err(UserError::InvalidRolePrimary(
                "Role Secondary is empty".to_owned(),
            ))
        } else {
            Ok(Self(Role::from_str(role)))
        }
    }
    pub fn into_inner(&self) -> &Role {
        &self.0
    }
    pub fn to_const(self) -> String {
        Role::from_const(&self.0)
    }
}
