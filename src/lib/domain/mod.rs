pub mod availability;
pub mod user;
pub mod week;

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Clone, Copy, Sequence)]
pub enum ScheduleDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl ScheduleDay {
    fn from_const(day: &ScheduleDay) -> String {
        match day {
            ScheduleDay::Monday => "Monday".to_owned(),
            ScheduleDay::Tuesday => "Tuesday".to_owned(),
            ScheduleDay::Wednesday => "Wednesday".to_owned(),
            ScheduleDay::Thursday => "Thursday".to_owned(),
            ScheduleDay::Friday => "Friday".to_owned(),
            ScheduleDay::Saturday => "Saturday".to_owned(),
            ScheduleDay::Sunday => "Sunday".to_owned(),
        }
    }
    pub fn from_str(day: &str) -> ScheduleDay {
        match day.to_lowercase().as_str() {
            "monday" => ScheduleDay::Monday,
            "tuesday" => ScheduleDay::Tuesday,
            "wednesday" => ScheduleDay::Wednesday,
            "thursday" => ScheduleDay::Thursday,
            "friday" => ScheduleDay::Friday,
            "saturday" => ScheduleDay::Saturday,
            "sunday" => ScheduleDay::Sunday,
            _ => unreachable!("Invalid day"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub enum ScheduleTime {
    StartAtOne,
    StartAtThree,
    StartAtFive,
    StartAtSix,
    FromOneToFive,
    FromThreeToFive,
    OnCallAtFive,
    OnCallAtSix,
    OnCallAtFiveStartAtSix,
    Free,
    None,
}
impl ScheduleTime {
    pub fn from_const(&self) -> String {
        match self {
            ScheduleTime::StartAtOne => "13".to_owned(),
            ScheduleTime::StartAtThree => "15".to_owned(),
            ScheduleTime::StartAtFive => "17".to_owned(),
            ScheduleTime::StartAtSix => "18".to_owned(),
            ScheduleTime::FromOneToFive => "13-17".to_owned(),
            ScheduleTime::FromThreeToFive => "15-17".to_owned(),
            ScheduleTime::OnCallAtFive => "(17)".to_owned(),
            ScheduleTime::OnCallAtSix => "(18)".to_owned(),
            ScheduleTime::OnCallAtFiveStartAtSix => "(17)18".to_owned(),
            ScheduleTime::Free => "free".to_owned(),
            ScheduleTime::None => "".to_owned(),
        }
    }
    pub fn from_str(s: &str) -> ScheduleTime {
        match s {
            "13" => ScheduleTime::StartAtOne,
            "15" => ScheduleTime::StartAtThree,
            "17" => ScheduleTime::StartAtFive,
            "18" => ScheduleTime::StartAtSix,
            "13-17" => ScheduleTime::FromOneToFive,
            "15-17" => ScheduleTime::FromThreeToFive,
            "(17)" => ScheduleTime::OnCallAtFive,
            "(18)" => ScheduleTime::OnCallAtSix,
            "(17)18" => ScheduleTime::OnCallAtFiveStartAtSix,
            "free" => ScheduleTime::Free,
            _ => ScheduleTime::None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Clone, Sequence)]
pub enum Role {
    Griller,
    Kitchen,
    Bar,
    Service,
    Management,
    Dishwasher,
    None,
    All,
}
impl Role {
    pub fn from_str(s: &str) -> Role {
        match s.to_lowercase().as_str() {
            "griller" => Role::Griller,
            "kitchen" => Role::Kitchen,
            "bar" => Role::Bar,
            "service" => Role::Service,
            "management" => Role::Management,
            "dishwasher" => Role::Dishwasher,
            "none" => Role::None,
            "all" => Role::All,
            _ => Role::None,
        }
    }
    pub fn from_const(&self) -> String {
        match *self {
            Role::Griller => "Griller".to_string(),
            Role::Kitchen => "Kitchen".to_string(),
            Role::Bar => "Bar".to_string(),
            Role::Service => "Service".to_string(),
            Role::Management => "Management".to_string(),
            Role::Dishwasher => "Dishwasher".to_string(),
            Role::None => "None".to_string(),
            Role::All => "All".to_string(),
        }
    }
}
