use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    pub name: String,
    pub time: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availabilities {
    pub monday: Vec<Availability>,
    pub tuesday: Vec<Availability>,
    pub wednesday: Vec<Availability>,
    pub thursday: Vec<Availability>,
    pub friday: Vec<Availability>,
    pub saturday: Vec<Availability>,
    pub sunday: Vec<Availability>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockData {
    pub admin: String,
    pub date: DateTime<Utc>,
    pub availabilities: Availabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanJson {
    id: String,
    user: String,
    day: String,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Day {
    fn from_str(day: &String) -> Day {
        match day.to_lowercase().as_str() {
            "monday" => Day::Monday,
            "tuesday" => Day::Tuesday,
            "wednesday" => Day::Wednesday,
            "thursday" => Day::Thursday,
            "friday" => Day::Friday,
            "saturday" => Day::Saturday,
            "sunday" => Day::Sunday,
            _ => unreachable!("Invalid day"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Time {
    StartAtOne,
    StartAtThree,
    StartAtFive,
    StartAtSix,
    StartOneEndFive,
    StartThreeEndFive,
    Custom(String),
    OnCall(String),
}
impl Time {
    pub fn from_str(s: &str) -> Time {
        match s {
            "13" => Time::StartAtOne,
            "15" => Time::StartAtThree,
            "17" => Time::StartAtFive,
            "18" => Time::StartAtSix,
            "13-17" => Time::StartOneEndFive,
            "15-17" => Time::StartThreeEndFive,
            _ => Time::Custom(s.to_owned()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    id: String,
    user: String,
    day: Day,
    time: Time,
}
impl Plan {
    fn new(plan: PlanJson) -> Plan {
        Plan {
            id: plan.id,
            user: plan.user,
            day: Day::from_str(&plan.day),
            time: Time::from_str(&plan.time),
        }
    }
}

pub fn get_mock_data(num: &str) -> Plan {
    let json = std::fs::read_to_string(format!("src/.mock/plan/plan{num}.json"))
        .expect("Failed to read file");

    // Parse the JSON into a PlanJson instance
    let plan_json: PlanJson = serde_json::from_str(json.as_str()).expect("Failed to parse JSON");

    // Create a PlanJson instance with extracted day and time
    let plan = Plan::new(plan_json);

    return plan;
}
