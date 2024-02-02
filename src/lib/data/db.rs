use crate::data::DbId;
use crate::domain::user::field;
use crate::domain::user::{User, UserError};

use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct UserJson {
    id: String,
    name: String,
    employee_id: String,
    admin: String,
    vast: String,
    active: String,
    min_days: String,
    max_days: String,
    role_primary: String,
    role_secondary: String,
}
impl TryFrom<UserJson> for User {
    type Error = UserError;

    fn try_from(user_json: UserJson) -> Result<User, Self::Error> {
        // Parse the user_id string into a DbId
        let id = DbId::from_str(&user_json.id)?;
        let admin = user_json
            .admin
            .parse::<bool>()
            .expect("Failed to parse admin");
        let vast = user_json
            .admin
            .parse::<bool>()
            .expect("Failed to parse vast");
        let active = user_json
            .admin
            .parse::<bool>()
            .expect("Failed to parse active");

        // Create a new User instance
        Ok(User {
            id: field::UserID::new(id),
            name: field::Name::new(user_json.name.as_str())?,
            employee_id: field::EmployeeID::new(user_json.employee_id.as_str())?,
            admin: field::Admin::new(admin),
            vast: field::Vast::new(vast),
            active: field::Active::new(active),
            min_days: field::MinDays::new(user_json.min_days.as_str())?,
            max_days: field::MaxDays::new(user_json.max_days.as_str())?,
            role_primary: field::RolePrimary::new(user_json.role_primary.as_str())?,
            role_secondary: field::RoleSecondary::new(user_json.role_secondary.as_str())?,
        })
    }
}

pub fn get_mock_user(id: &str) -> Result<User, UserError> {
    let json = std::fs::read_to_string(format!("src/.mock/users/user{id}.json"))
        .expect("Failed to read user json file");

    // Parse the JSON into a PlanJson instance
    let user_json: UserJson =
        serde_json::from_str(json.as_str()).expect("Failed to parse users JSON");

    let user = User::try_from(user_json);

    return user;
}
