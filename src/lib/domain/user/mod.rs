pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("Invalid name: {0}")]
    InvalidEmployeeID(String),
    #[error("Invalid admin: {0}")]
    InvalidAdmin(String),
    #[error("Invalid vast: {0}")]
    InvalidVast(String),
    #[error("Invalid admin: {0}")]
    InvalidActive(String),
    #[error("Invalid vast: {0}")]
    InvalidMinDays(String),
    #[error("Invalid vast: {0}")]
    InvalidMaxDays(String),
    #[error("Invalid admin: {0}")]
    InvalidRolePrimary(String),
    #[error("Invalid vast: {0}")]
    InvalidRoleSecondary(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct user {
    id: uuid,
    name: field::Name,
    employee_id: field::EmployeeID,
    admin: field::Admin,
    vast: field::Vast,
    active: field::Active,
    min_days: field::MinDays,
    max_days: field::MaxDays,
    role_primary: field::RolePrimary,
    role_secondary: field::RoleSecondary,
}
