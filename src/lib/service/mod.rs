pub mod lib;
pub mod scheduler;

use crate::data::db;
use crate::web;

use self::scheduler::WeekSchedule;

#[derive(Debug)]
pub struct Logic {
    manager: u8,
    griller: u8,
    bar: u8,
    dishwashers: u8,
    servers: u8,
}

pub fn schedule_setup() -> WeekSchedule {
    // 1. Get all the users from the database
    let all_users = db::get_all_mock_users(10);

    // 2. Get all the user availability for the week
    let all_availability = web::get_all_mock_data(70);

    // 3. Get the schedule logic
    let schedule_logic = Logic {
        manager: 1,
        griller: 1,
        bar: 1,
        dishwashers: 1,
        servers: 3,
    };

    // Get the schedule for the week
    let week = scheduler::calc_schedule_week(&all_availability, &all_users, &schedule_logic);

    return week;
}
