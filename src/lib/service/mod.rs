pub mod lib;
pub mod scheduler;

use crate::data::db;
use crate::domain::ScheduleTime;
use crate::web;

use self::scheduler::WeekSchedule;

#[derive(Debug)]
pub struct Logic {
    manager: Vec<ScheduleTime>,
    griller: Vec<ScheduleTime>,
    kitchen: Vec<ScheduleTime>,
    bar: Vec<ScheduleTime>,
    dishwashers: Vec<ScheduleTime>,
    servers: Vec<ScheduleTime>,
}

pub fn schedule_setup() -> WeekSchedule {
    // 1. Get all the users from the database
    let all_users = db::get_all_mock_users(10);

    // 2. Get all the user availability for the week
    let all_availability = web::get_all_mock_data(70);

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
    let week = scheduler::calc_schedule_week(&all_availability, &all_users, &schedule_logic);

    return week;
}
