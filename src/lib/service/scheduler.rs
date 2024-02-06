use super::lib;

use crate::data::DbId;
use crate::domain::availability::field;
use crate::domain::availability::{self, Availability};
use crate::domain::user::User;
use crate::domain::{week, Role, ScheduleDay, ScheduleTime};

use crate::service::Logic;

use std::str::FromStr;

use enum_iterator::all;

#[derive(Debug, Clone)]
pub struct Employee {
    id: availability::field::AvailabilityId,
    name: availability::field::Name,
    time_available: availability::field::Time,
    time_given: availability::field::Time,
    chosen: u8,
}

#[derive(Debug, Clone)]
pub struct Employees {
    manager: Vec<Employee>,
    griller: Vec<Employee>,
    kitchen: Vec<Employee>,
    bar: Vec<Employee>,
    dishwashers: Vec<Employee>,
    servers: Vec<Employee>,
}

#[derive(Debug, Clone)]
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
    //* Sorting lists
    // 1. Get list of all users with Management Role from users in DB
    let list_vast_managers = user_list
        .clone()
        .iter()
        .cloned()
        .filter(|user| {
            user.vast.into_inner() == &true && user.role_secondary.into_inner() == role
                || user.role_primary.into_inner() == role
                || user.role_secondary.into_inner() == &Role::All
        })
        .collect::<Vec<User>>();

    // 2. Get list of all available users on a given day from available users
    let day_available_users = list
        .iter()
        .cloned()
        .filter(|availability| availability.day.into_inner() == day)
        .collect::<Vec<availability::Availability>>();

    // 3. Get list of available roles on a given day from available users on that day
    let monday_available_managers = {
        let mut new_vec = vec![];

        for manager in list_vast_managers.clone() {
            for user in day_available_users.clone() {
                if user.user_id.to_the_string() == manager.id.to_the_string() {
                    new_vec.push((user).clone())
                }
            }
        }
        new_vec
    };

    // 1. List of managers
    // let list_managers = list_all_management_users;
    // 2. List of managers with vast = true && chosen < max day
    let list_vast_users = list_vast_managers;
    // 3. Available managers
    let list_all_available_users = monday_available_managers;

    //? <--------------------
    //* Logic to distribute the available users

    // Process:
    // check if list of available managers is empty
    if list_all_available_users.is_empty() {
        // println!("No available users on {:?} for {:?}", day, role);
        let generate_needed_manager = lib::get_user_with_highest_max_days(&list_vast_users, role);

        // Check if the day is Monday to adjust the start time
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
        // Assuming the type of elements in list_all_available_users is User
        let mut the_manager_list: Vec<Availability> = Vec::new();

        // Sort list_all_available_users in-place by time
        let list_all_available_users = lib::bubble_sort(&mut list_all_available_users.to_owned());

        // Assuming you want to clone elements from list_all_available_users to the_manager_list
        for i in 0..logic.len().to_owned() {
            // for user in list_all_available_users.iter() {
            // println!("Processing user");
            the_manager_list.push(list_all_available_users[i].clone());
            // }
        }
        return the_manager_list;

        // Process the available list x amount of time
    } else {
        let mut the_manager_list = Vec::new();
        for _ in 0..logic.len().to_owned() {
            let num = lib::get_random_number(list_all_available_users.len());
            let manager = list_all_available_users.get(num).unwrap();
            the_manager_list.push(manager.to_owned());
        }
        return the_manager_list;
    }
}

//? -------------------->

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

    let mut list = Vec::new();

    for i in 0..users.len() {
        let new_employee = Employee {
            id: users[i].user_id.to_owned(),
            name: users[i].name.to_owned(),
            time_available: users[i].time.to_owned(),
            time_given: match role {
                Role::Dishwasher | Role::Bar => {
                    availability::field::Time::create(ScheduleTime::StartAtSix)
                }
                Role::Management => availability::field::Time::create(ScheduleTime::StartAtThree),
                _ => match users[i].time.into_inner() {
                    ScheduleTime::Available => {
                        availability::field::Time::create(logic[i].to_owned())
                    }
                    _ => {
                        if logic[i] > users[i].time.into_inner() {
                            availability::field::Time::create(logic[i].to_owned())
                        } else {
                            availability::field::Time::create(users[i].time.into_inner())
                        }
                    }
                },
            },
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
    // Create the vector to hold the roles
    let mut roles: Vec<Vec<Employee>> = Vec::new();

    // Using the enum iterator to loop through the roles
    for role in all::<Role>() {
        if role != Role::All || role != Role::None {
            let position = calc_schedule_role(
                all_availability,
                all_users,
                &schedule_logic.manager,
                &role,
                day,
            );
            roles.push(position)
        }
    }

    // Create the day schedule with the roles
    let the_day = DaySchedule {
        day: week::field::Days::create(day.to_owned()),
        employees: Employees {
            // Add the role from roles to the employees
            servers: roles.pop().expect("could not pop server roles"),
            dishwashers: roles.pop().expect("could not pop dishwasher roles"),
            bar: roles.pop().expect("could not pop bar roles"),
            kitchen: roles.pop().expect("could not pop kitchen roles"),
            griller: roles.pop().expect("could not pop griller roles"),
            manager: roles.pop().expect("could not pop manager roles"),
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
    let mut week: Vec<DaySchedule> = Vec::new();

    for day in all::<ScheduleDay>() {
        let the_day = calc_schedule_day(&all_availability, &all_users, &schedule_logic, &day);
        week.push(the_day)
    }

    let week = WeekSchedule {
        weekly_id: week::field::WeeklyId::new(),
        sunday: week.pop().expect("could not pop sunday"),
        saturday: week.pop().expect("could not pop saturday"),
        friday: week.pop().expect("could not pop friday"),
        thursday: week.pop().expect("could not pop thursday"),
        wednesday: week.pop().expect("could not pop wednesday"),
        tuesday: week.pop().expect("could not pop tuesday"),
        monday: week.pop().expect("could not pop monday"),
    };
    return week;
}
