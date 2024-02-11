pub mod lib;
pub mod scheduler;
pub mod setup;

use crate::data::db;
use crate::domain::ScheduleTime;
use crate::web;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Logic {
    pub manager: Vec<ScheduleTime>,
    pub griller: Vec<ScheduleTime>,
    pub kitchen: Vec<ScheduleTime>,
    pub bar: Vec<ScheduleTime>,
    pub dishwashers: Vec<ScheduleTime>,
    pub servers: Vec<ScheduleTime>,
}

pub fn schedule_setup() {
    // 1. Get all the users from the database
    let all_users = db::get_all_mock_users(10);

    // 2. Get all the user availability for the week
    let all_availability = web::get_all_mock_data(70);

    // 1.1 Create a list of weekly chosen users
    let mut chosen_users: HashMap<String, u8> = setup::create_hashmap_tracker(&all_users);

    let weekly_schedule = setup::create_info_matrix(&all_users, &all_availability);

    // 3. Get the schedule logic
    let schedule_logic = Logic {
        manager: vec![ScheduleTime::StartAtThree],
        griller: vec![ScheduleTime::StartAtThree],
        kitchen: vec![ScheduleTime::StartAtThree, ScheduleTime::StartAtSix],
        bar: vec![ScheduleTime::StartAtSix],
        dishwashers: vec![ScheduleTime::StartAtSix],
        servers: vec![
            ScheduleTime::StartAtThree,
            ScheduleTime::StartAtFive,
            ScheduleTime::StartAtSix,
            ScheduleTime::OnCallAtSix,
        ],
    };

    // Get the schedule for the week
    let week = scheduler::calc_schedule_week(
        weekly_schedule,
        schedule_logic,
        &mut chosen_users,
        all_users,
    );

    return week;
}
