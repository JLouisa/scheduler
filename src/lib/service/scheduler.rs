use super::setup::InfoMatrix;

use crate::domain::availability::{self};
use crate::domain::{week, ScheduleTime};
use crate::service::Logic;

use std::collections::HashMap;

// use enum_iterator::all;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Employee {
    id: availability::field::AvailabilityId,
    name: availability::field::Name,
    time_available: availability::field::Time,
    time_given: availability::field::Time,
}
impl Employee {
    pub fn no_user() -> Self {
        Self {
            id: availability::field::AvailabilityId::default(),
            name: availability::field::Name::new("No User").expect("cannot parse No User name"),
            time_available: availability::field::Time::create(ScheduleTime::None),
            time_given: availability::field::Time::create(ScheduleTime::None),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Employees {
    pub manager: Option<Vec<Employee>>,
    pub griller: Option<Vec<Employee>>,
    pub kitchen: Option<Vec<Employee>>,
    pub bar: Option<Vec<Employee>>,
    pub dishwashers: Option<Vec<Employee>>,
    pub servers: Option<Vec<Employee>>,
}

#[derive(Debug, Clone)]
pub struct DaySchedule {
    pub day: week::field::Days,
    pub employees: Employees,
}

#[derive(Debug)]
pub struct WeekSchedule {
    pub weekly_id: week::field::WeeklyId,
    pub monday: DaySchedule,
    pub tuesday: DaySchedule,
    pub wednesday: DaySchedule,
    pub thursday: DaySchedule,
    pub friday: DaySchedule,
    pub saturday: DaySchedule,
    pub sunday: DaySchedule,
}

///* Calculate the schedule for week of the staff
pub fn calc_schedule_week(
    sorted_week: InfoMatrix,
    schedule_logic: Logic,
    chosen_users: &mut HashMap<String, u8>,
) {
    let griller = sorted_week.monday.roles.griller;
    println!("Griller: {:?}", griller);
    println!("");

    let management = sorted_week.monday.roles.management;
    println!("Management: {:?}", management);
    println!("");
}
