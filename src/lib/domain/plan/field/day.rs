use crate::domain::PlanError::InvalidDay;
use serde::{Deserialize, Serialize};

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
    fn from_str(day: &str) -> Day {
        match day.to_lowercase() {
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
    fn from_day(day: Day) -> String {
        match day {
            Day::Monday => "Monday".to_owned(),
            Day::Tuesday => "Tuesday".to_owned(),
            Day::Wednesday => "Wednesday".to_owned(),
            Day::Thursday => "Thursday".to_owned(),
            Day::Friday => "Friday".to_owned(),
            Day::Saturday => "Saturday".to_owned(),
            Day::Sunday => "Sunday".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Day(String);
impl Day {
    pub fn new(day: &str) -> Result<Self, InvalidDay> {
        if day.trim().is_empty() {
            Err(PlanError::InvalidDay("Day is empty".to_owned()))
        } else {
            Ok(Self(day.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn to_const(self) -> Day {
        Day::from_str(self.0.as_str())
    }
}
