use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

use crate::data::DbId;
use crate::domain::availability::{field, Availability, AvailabilityError};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanJson {
    user_id: String,
    name: String,
    day: String,
    time: String,
}

impl TryFrom<PlanJson> for Availability {
    type Error = AvailabilityError;
    fn try_from(plan: PlanJson) -> Result<Availability, Self::Error> {
        // Parse the user_id string into a DbId
        let user_id = DbId::from_str(&plan.user_id)?;

        // Create a new Availability instance
        Ok(Availability {
            user_id: field::AvailabilityId::new(user_id),
            weekly_id: field::WeeklyId::new(),
            name: field::Name::new(plan.name.as_str())?,
            day: field::Days::new(plan.day.as_str()),
            time: field::Time::new(plan.time.as_str()),
        })
    }
}

pub fn get_mock_data(num: &str) -> Result<Availability, AvailabilityError> {
    let json = std::fs::read_to_string(format!("src/.mock/plan/plan{num}.json"))
        .expect("Failed to read plan json file");

    // Parse the JSON into a PlanJson instance
    let plan_json: PlanJson = serde_json::from_str(json.as_str()).expect("Failed to parse JSON");

    // Create a PlanJson instance with extracted day and time
    let plan = Availability::try_from(plan_json);

    return plan;
}

pub fn get_all_mock_data(num: u16) -> Vec<Availability> {
    let mut availability_list: Vec<Availability> = Vec::new();
    for i in 1..=num {
        let plan = get_mock_data(&i.to_string().as_str()).expect("Failed to get mock data");
        availability_list.push(plan);
    }
    return availability_list;
}
