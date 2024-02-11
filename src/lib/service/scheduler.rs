use super::setup::InfoMatrix;

use crate::data::DbId;
use crate::domain::availability::{self, AvailabilitySpot};
use crate::domain::user::User;
use crate::domain::{week, Role, ScheduleTime};
use crate::service::setup;
use crate::service::Logic;

use std::collections::HashMap;
use std::str::FromStr;

// use enum_iterator::all;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Employee {
    id: availability::field::AvailabilityId,
    name: availability::field::Name,
    time_available: availability::field::Time,
    time_given: availability::field::Time,
}
impl Employee {
    pub fn create(
        id: &str,
        name: &str,
        time_available: ScheduleTime,
        time_given: ScheduleTime,
    ) -> Self {
        Self {
            id: availability::field::AvailabilityId::new(DbId::from_str(id).unwrap()),
            name: availability::field::Name::new(name).unwrap(),
            time_available: availability::field::Time::create(time_available),
            time_given: availability::field::Time::create(time_given),
        }
    }
    pub fn no_user() -> Self {
        Self {
            id: availability::field::AvailabilityId::default(),
            name: availability::field::Name::new("No User").expect("cannot parse No User name"),
            time_available: availability::field::Time::create(ScheduleTime::None),
            time_given: availability::field::Time::create(ScheduleTime::None),
        }
    }
    pub fn new(available: &Option<AvailabilitySpot>, the_time: ScheduleTime) -> Self {
        if available.is_some() {
            Self::create(
                available
                    .clone()
                    .expect("unwrapping available ID in Employee impl.")
                    .user_id
                    .to_the_string()
                    .as_str(),
                available
                    .clone()
                    .expect("unwrapping available name in Employee impl.")
                    .name
                    .as_str(),
                available
                    .clone()
                    .expect("unwrapping available time in Employee impl.")
                    .time
                    .into_inner(),
                the_time,
            )
        } else {
            Self::no_user()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Employees {
    pub manager: Vec<Employee>,
    pub griller: Vec<Employee>,
    pub kitchen: Vec<Employee>,
    pub bar: Vec<Employee>,
    pub dishwashers: Vec<Employee>,
    pub servers: Vec<Employee>,
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
    all_users: Vec<User>,
) {
}

pub fn calc_schedule_week_manager(
    manager_list: &Vec<Option<AvailabilitySpot>>,
    schedule_logic: Logic,
    chosen_users: &mut HashMap<String, u8>,
    all_users: &Vec<User>,
) -> Vec<Option<AvailabilitySpot>> {
    let mut new_manager_list = Vec::new();
    let mut hold_list = Vec::new();
    let length_manager_list = schedule_logic.manager.len();
    let users_by_role = setup::filter_all_user_on_role(all_users, &Role::Management);

    for user in manager_list.iter() {
        if user.is_some() {
            let available = user
                .clone()
                .expect("Unable to unwrap user in calc_schedule_week_manager");
            let chosen = chosen_users.get(&available.user_id.to_the_string());
            let found_user = users_by_role
                .iter()
                .find(|&x| x.id.to_the_string() == available.user_id.to_the_string());

            if found_user.is_some() && chosen.is_some() {
                let found_user =
                    found_user.expect("Unable to unwrap found_user in calc_schedule_week_manager");
                let chosen = chosen.expect("Unable to unwrap chosen in calc_schedule_week_manager");

                if *chosen < found_user.max_days.ref_into_inner()
                    && found_user.vast.into_inner() == &true
                {
                    new_manager_list.push(user.clone());
                    chosen_users.insert(available.user_id.to_the_string(), chosen + 1);
                }
            } else if let Some(chosen) = chosen {
                if *chosen == 0 && new_manager_list.len() < length_manager_list {
                    new_manager_list.push(user.clone());
                    chosen_users.insert(available.user_id.to_the_string(), chosen + 1);
                }
            } else {
                hold_list.push(user.clone())
            }
            if new_manager_list.len() >= length_manager_list {
                break;
            }
        }
    }

    return new_manager_list;
}
