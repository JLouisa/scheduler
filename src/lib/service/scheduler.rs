use super::lib;

use crate::data::DbId;
use crate::domain::availability::field;
use crate::domain::availability::{self, Availability};
use crate::domain::user::User;
use crate::domain::{week, Role, ScheduleDay, ScheduleTime};
use crate::service::Logic;

use std::str::FromStr;

#[derive(Debug)]
pub struct Employee {
    id: availability::field::AvailabilityId,
    name: availability::field::Name,
    time: availability::field::Time,
    chosen: u8,
}

#[derive(Debug)]
pub struct Employees {
    manager: Vec<Employee>,
    griller: Vec<Employee>,
    kitchen: Vec<Employee>,
    bar: Vec<Employee>,
    dishwashers: Vec<Employee>,
    servers: Vec<Employee>,
}

#[derive(Debug)]
pub struct DaySchedule {
    day: week::field::Days,
    employees: Employees,
}

#[derive(Debug)]
pub struct WeekSchedule {
    weekly_id: week::field::WeeklyId,
    monday: DaySchedule,
    tuesday: DaySchedule,
    wednesday: DaySchedule,
    thursday: DaySchedule,
    friday: DaySchedule,
    saturday: DaySchedule,
    sunday: DaySchedule,
}

pub fn get_user(
    list: &Vec<availability::Availability>,
    user_list: &Vec<User>,
    logic: &Vec<ScheduleTime>,
    role: &Role,
    day: &ScheduleDay,
) -> Vec<availability::Availability> {
    // 1. Get list of all users with Management Role from users in DB
    let binding1 = user_list.clone();
    // let list_all_management_users = binding1
    //     .iter()
    //     .cloned()
    //     .filter(|user| {
    //         user.role_secondary.into_inner() == role || user.role_primary.into_inner() == role
    //     })
    //     .collect::<Vec<User>>();

    // 2. Get list of Managers with vast = true from all users with Management Role
    let list_vast_managers = binding1
        .iter()
        .cloned()
        .filter(|user| {
            user.vast.into_inner() == &true && user.role_secondary.into_inner() == role
                || user.role_primary.into_inner() == role
        })
        .collect::<Vec<User>>();

    // 1. Get list of all available users on Monday from available users
    let monday_available_users = list
        .iter()
        .cloned()
        .filter(|availability| availability.day.into_inner() == day)
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
    // let list_managers = list_all_management_users;
    // 2. List of managers with vast = true && chosen < max day
    let list_vast_users = list_vast_managers;
    // 3. Available managers
    let list_all_available_users = monday_available_managers;

    // Process:
    // check if list of available managers is empty
    if list_all_available_users.is_empty() {
        println!("No available users on {:?} for {:?}", day, role);
        let generate_needed_manager = lib::get_user_with_highest_max_days(&list_vast_users);

        // Check if the day is Monday to adjust the time
        let the_time = if day == &ScheduleDay::Monday {
            &ScheduleTime::StartAtOne
        } else {
            &ScheduleTime::StartAtThree
        };

        // Create new availability for the needed manager
        let new_available_manager = availability::Availability {
            user_id: field::AvailabilityId::new(
                DbId::from_str(generate_needed_manager.id.to_the_string().as_str())
                    .expect("could not create id"),
            ),
            weekly_id: field::WeeklyId::new(),
            name: field::Name::new(generate_needed_manager.name.as_str())
                .expect("could not create name"),
            day: field::Days::create(day.to_owned()),
            time: field::Time::create(the_time.to_owned()),
        };
        return vec![new_available_manager];

        // check if the list of vast managers is empty
    } else if list_vast_users.is_empty() {
        let mut the_manager_list = vec![];
        if role == &Role::Service {
            for i in 0..logic.len().to_owned() {
                println!("Round {}", i + 1);
                let num = lib::get_random_number(list_all_available_users.len());
                let manager = list_all_available_users
                    .get(num)
                    .expect("could not get manager");
                the_manager_list.push(manager.to_owned());
            }
        } else {
        }
        return the_manager_list;

        // Process the available list x amount of time
    } else {
        let mut the_manager_list = vec![];
        for _ in 0..logic.len().to_owned() {
            let num = lib::get_random_number(list_all_available_users.len());
            let manager = list_all_available_users.get(num).unwrap();
            the_manager_list.push(manager.to_owned());
        }
        return the_manager_list;
    }
}

///* Calculate the schedule for a role of the user
pub fn calc_schedule_role(
    available_list: &Vec<Availability>,
    user_list: &Vec<User>,
    logic: &Vec<ScheduleTime>,
    role: &Role,
    day: &ScheduleDay,
) -> Vec<Employee> {
    // Process Manager
    let users = get_user(available_list, user_list, logic, role, day);

    let the_time = if role == &Role::Dishwasher {
        availability::field::Time::create(ScheduleTime::StartAtSix)
    } else {
        availability::field::Time::create(ScheduleTime::StartAtThree)
    };

    let mut list = vec![];
    for user in users {
        let new_employee = Employee {
            id: user.user_id.to_owned(),
            name: user.name.to_owned(),
            time: the_time.to_owned(),
            chosen: 0,
        };
        list.push(new_employee);
    }

    return list;
}

///* Calculate the schedule for a day of the week
pub fn calc_schedule_day(
    all_availability: &Vec<Availability>,
    all_users: &Vec<User>,
    schedule_logic: &Logic,
    day: &ScheduleDay,
) -> DaySchedule {
    let the_day = DaySchedule {
        day: week::field::Days::create(day.to_owned()),
        employees: Employees {
            manager: calc_schedule_role(
                all_availability,
                all_users,
                &schedule_logic.manager,
                &Role::Management,
                day,
            ),
            griller: calc_schedule_role(
                all_availability,
                all_users,
                &schedule_logic.griller,
                &Role::Griller,
                day,
            ),
            kitchen: calc_schedule_role(
                all_availability,
                all_users,
                &schedule_logic.kitchen,
                &Role::Kitchen,
                day,
            ),
            bar: calc_schedule_role(
                all_availability,
                all_users,
                &schedule_logic.bar,
                &Role::Bar,
                day,
            ),
            dishwashers: calc_schedule_role(
                all_availability,
                all_users,
                &schedule_logic.dishwashers,
                &Role::Dishwasher,
                day,
            ),
            servers: calc_schedule_role(
                all_availability,
                all_users,
                &schedule_logic.servers,
                &Role::Service,
                day,
            ),
        },
    };
    return the_day;
}

///* Calculate the schedule for week of the staff
pub fn calc_schedule_week(
    all_availability: &Vec<Availability>,
    all_users: &Vec<User>,
    schedule_logic: &Logic,
) -> WeekSchedule {
    let week = WeekSchedule {
        weekly_id: week::field::WeeklyId::new(),
        monday: calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &ScheduleDay::Monday,
        ),
        tuesday: calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &ScheduleDay::Tuesday,
        ),
        wednesday: calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &ScheduleDay::Wednesday,
        ),
        thursday: calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &ScheduleDay::Thursday,
        ),
        friday: calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &ScheduleDay::Friday,
        ),
        saturday: calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &ScheduleDay::Saturday,
        ),
        sunday: calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &ScheduleDay::Sunday,
        ),
    };
    return week;
}
