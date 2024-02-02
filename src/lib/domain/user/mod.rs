pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Invalid employee ID: {0}")]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: field::UserID,
    pub name: field::Name,
    pub employee_id: field::EmployeeID,
    pub admin: field::Admin,
    pub vast: field::Vast,
    pub active: field::Active,
    pub min_days: field::MinDays,
    pub max_days: field::MaxDays,
    pub role_primary: field::RolePrimary,
    pub role_secondary: field::RoleSecondary,
}
