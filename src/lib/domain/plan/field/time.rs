use crate::domain::plan::PlanError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Time {
    StartAtOne,
    StartAtThree,
    StartAtFive,
    StartAtSix,
    FromOneToFive,
    FromThreeToFive,
    Custom(String),
    OnCall(String),
}
impl Time {
    pub fn to_num(&self) -> String {
        match self {
            Time::StartAtOne => "13".to_owned(),
            Time::StartAtThree => "15".to_owned(),
            Time::StartAtFive => "17".to_owned(),
            Time::StartAtSix => "18".to_owned(),
            Time::FromOneToFive => "13-17".to_owned(),
            Time::FromThreeToFive => "15-17".to_owned(),
            Time::Custom(n) => n.to_owned(),
            Time::OnCall(n) => n.to_owned(),
        }
    }
    pub fn from_str(s: &str) -> Time {
        match s {
            "13" => Time::StartAtOne,
            "15" => Time::StartAtThree,
            "17" => Time::StartAtFive,
            "18" => Time::StartAtSix,
            "13-17" => Time::FromOneToFive,
            "15-17" => Time::FromThreeToFive,
            _ => Time::Custom(s.to_owned()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Time(String);
impl Time {
    pub fn new(Time: &str) -> Result<Self, PlanError> {
        if Time.trim().is_empty() {
            Err(PlanError::InvalidTime("Time is empty".to_owned()))
        } else {
            Ok(Self(Time.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_const(self) -> Time {
        Time::from_str(self.0.as_str())
    }
}
