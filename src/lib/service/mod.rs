use crate::data::{db, DbId};
use crate::domain::availability;
use crate::domain::availability::field;
use crate::domain::user::User;
use crate::domain::{week, Role, ScheduleDay, ScheduleTime};
use crate::web;

use std::str::FromStr;

use rand::Rng;

#[derive(Debug)]
struct Employee {
    id: availability::field::AvailabilityId,
    name: availability::field::Name,
    time: availability::field::Time,
    chosen: u8,
}

#[derive(Debug)]
struct Employees {
    manager: Employee,
    griller: Employee,
    bar: Employee,
    dishwashers: Vec<Employee>,
    servers: Vec<Employee>,
}

#[derive(Debug)]
struct MondaySchedule {
    weekly_id: String,
    day: week::field::Days,
    employees: Employees,
}

#[derive(Debug)]
struct Logic {
    manager: u8,
    griller: u8,
    bar: u8,
    dishwashers: u8,
    servers: u8,
}

fn get_random_number(num: usize) -> usize {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..num);

    return random_index;
}

fn get_managers(
    list: Vec<availability::Availability>,
    user_list: &Vec<User>,
    logic: &u8,
) -> Vec<availability::Availability> {
    // 1. Get list of all users with Management Role from users in DB
    let binding1 = user_list.clone();
    let list_all_management_users = binding1
        .iter()
        .filter(|user| {
            user.role_secondary.into_inner() == &Role::Management
                || user.role_primary.into_inner() == &Role::Management
        })
        .collect::<Vec<&User>>();

    // 2. Get list of Managers with vast = true from all users with Management Role
    let binding2 = list_all_management_users.clone();
    let list_vast_managers = binding2
        .iter()
        .filter(|user| user.vast.into_inner() == &true)
        .collect::<Vec<&&User>>();

    // 1. Get list of all available users on Monday from available users
    let monday_available_users = list
        .iter()
        .cloned()
        .filter(|availability| availability.day.into_inner() == &ScheduleDay::Monday)
        .collect::<Vec<availability::Availability>>();

    // 2. Get list of available managers on Monday from available users on Monday
    let monday_available_managers = {
        let mut new_vec = vec![];

        for manager in list_vast_managers.clone() {
            for user in monday_available_users.clone() {
                if user.user_id.to_the_string() == manager.id.to_the_string() {
                    new_vec.push((user).clone())
                }
            }
        }
        new_vec
    };

    for manager in monday_available_managers.iter() {
        println!("User from managers: {:?}", manager);
    }

    // 1. List of managers
    let list_managers = list_all_management_users;
    // 2. List of managers with vast = true && chosen < max day
    let list_vast_managers = list_vast_managers;
    // 3. Available managers
    let list_all_available_managers = monday_available_managers;

    // Process:
    // 4. Compare id of Manager with vast to available managers on Monday
    let binding3: Vec<&&User> = list_vast_managers.clone();
    if list_all_available_managers.len() < 1 {
        let generate_needed_manager = binding3.iter().max_by_key(|item| item.max_days).unwrap();
        let new_available_manager = availability::Availability {
            user_id: field::AvailabilityId::new(
                DbId::from_str(generate_needed_manager.id.to_the_string().as_str())
                    .expect("could not create id"),
            ),
            weekly_id: field::WeeklyId::new(),
            name: field::Name::new(generate_needed_manager.name.as_str())
                .expect("could not create name"),
            day: field::Days::create(ScheduleDay::Monday),
            time: field::Time::create(ScheduleTime::StartAtOne),
        };
        return vec![new_available_manager];
    } else {
        // do Something else
        // let len = list_vast_managers.len();
        // let get_random_manager = list_vast_managers[get_random_number(len)];
    }
}

pub fn calc_monday_schedule() {
    let print_lines = false;
    // The maximum number of employees needed for each position
    let schedule_logic = Logic {
        manager: 1,
        griller: 1,
        bar: 1,
        dishwashers: 1,
        servers: 3,
    };

    //* Global
    // 1. All the user availability for the week
    let all_availability = web::get_all_mock_data(6);

    // 2. All the users from the database
    let all_users = db::get_all_mock_users(6);

    let manager = get_managers(all_availability, &all_users, &schedule_logic.manager);
}
