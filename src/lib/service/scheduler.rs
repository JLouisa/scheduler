use super::lib;

use crate::data::DbId;
use crate::domain::availability::field;
use crate::domain::availability::{self, Availability};
use crate::domain::user::User;
use crate::domain::{week, Role, ScheduleDay, ScheduleTime};
use crate::service::Logic;

use std::collections::HashMap;
use std::str::FromStr;

use enum_iterator::all;

#[derive(Debug, Clone)]
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
    manager: Option<Vec<Employee>>,
    griller: Option<Vec<Employee>>,
    kitchen: Option<Vec<Employee>>,
    bar: Option<Vec<Employee>>,
    dishwashers: Option<Vec<Employee>>,
    servers: Option<Vec<Employee>>,
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

pub fn get_users(
    list: &Vec<availability::Availability>,
    user_list: &Vec<User>,
    logic: &Vec<ScheduleTime>,
    role: &Role,
    day: &ScheduleDay,
    chosen_users: &mut HashMap<String, u8>,
) -> Vec<Option<availability::Availability>> {
    // Debug
    println!("In section 1");

    //* Sorting lists
    // 1. Get list of all users with Management Role from users in DB
    let list_vast_users = lib::sort_vast_users(user_list, role);

    // 2. Get list of all available users on a given day from available users
    let day_available_users = lib::sort_available_users(list, day);

    // 3. Get list of available roles on a given day from available users on that day
    let list_all_available_users =
        lib::sort_available_users_on_role(&list_vast_users, &day_available_users);

    //? <--------------------

    //* Logic to distribute the available users
    // Process:
    //? check if list of available users is empty
    if list_all_available_users.is_empty() {
        let generate_needed_user = lib::get_user_with_highest_max_days(&list_vast_users, role);

        match generate_needed_user {
            Some(g_user) => {
                // Increase the chosen count for the user
                lib::increase_chosen_user_count(g_user.id.to_the_string().as_str(), chosen_users);

                // Check if the day is Monday to adjust the start time
                let the_time = if day == &ScheduleDay::Monday {
                    &ScheduleTime::StartAtOne
                } else {
                    &ScheduleTime::StartAtThree
                };

                // Create new availability for the needed manager
                let new_available_manager = availability::Availability {
                    user_id: field::AvailabilityId::new(
                        DbId::from_str(g_user.id.to_the_string().as_str())
                            .expect("could not create id"),
                    ),
                    weekly_id: field::WeeklyId::new(),
                    name: field::Name::new(g_user.name.as_str()).expect("could not create name"),
                    day: field::Days::create(day.to_owned()),
                    time: field::Time::create(the_time.to_owned()),
                };
                return vec![Some(new_available_manager)];
            }
            None => return vec![None],
        }

        //? check if the list of vast managers is empty
    } else if list_vast_users.is_empty() {
        // Debug
        println!("In section 2");

        // Assuming the type of elements in list_all_available_users is User
        let mut the_user_role_list: Vec<Option<Availability>> = Vec::new();

        // Sort list_all_available_users in-place by time
        let list_all_available_users: Vec<Availability> =
            lib::bubble_sort(&mut list_all_available_users.to_owned());

        // Sort the list of available users based on chosen count
        let list_all_available_users: Vec<Option<Availability>> = lib::sort_lowest_to_highest_count(
            list_all_available_users,
            chosen_users,
            list_vast_users,
        );

        // Process the available list (x logic per role) amount of time
        for i in 0..logic.len().to_owned() {
            match &list_all_available_users[i] {
                Some(user) => {
                    the_user_role_list.push(Some(user.clone()));
                }
                None => the_user_role_list.push(None),
            }
        }
        return the_user_role_list;

        //? If the list of available Vast users is not empty
    } else {
        // Assuming the type of elements in list_all_available_users is User
        let mut the_user_role_list: Vec<Option<Availability>> = Vec::new();

        // Sort list_all_available_users by time
        let list_all_available_users: Vec<Availability> =
            lib::bubble_sort(&mut list_all_available_users.to_owned());

        // Sort the list of available users based on chosen count
        let list_all_available_users = lib::sort_lowest_to_highest_count(
            list_all_available_users,
            chosen_users,
            list_vast_users,
        );

        // Process the available list (x logic per role) amount of time
        for i in 0..logic.len().to_owned() {
            match &list_all_available_users[i] {
                Some(user) => {
                    the_user_role_list.push(Some(user.clone()));
                }
                None => the_user_role_list.push(None),
            }
        }
        return the_user_role_list;
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
    chosen_users: &mut HashMap<String, u8>,
) -> Vec<Employee> {
    // println!("All users");
    // println!("{:?}", user_list);

    // println!("All availabilities");
    // println!("{:?}", available_list);

    // Process Manager
    let users: Vec<Option<Availability>> =
        get_users(available_list, user_list, logic, role, day, chosen_users);

    println!("After get users:");
    println!("{:?}", users);

    let mut list = Vec::new();

    // Debug
    println!("In section 3");

    for i in 0..logic.len() {
        match &users[i] {
            Some(u_users) => {
                let new_employee = Employee {
                    id: u_users.user_id.to_owned(),
                    name: u_users.name.to_owned(),
                    time_available: u_users.time.to_owned(),
                    time_given: match role {
                        // If role is Dishwasher or Bar
                        Role::Dishwasher | Role::Bar => {
                            availability::field::Time::create(ScheduleTime::StartAtSix)
                        }
                        // If role is Management
                        Role::Management => {
                            availability::field::Time::create(ScheduleTime::StartAtThree)
                        }
                        // Any other roles and will now match on time
                        _ => match u_users.time.into_inner() {
                            // If the time is Available
                            ScheduleTime::Available => {
                                availability::field::Time::create(logic[i].to_owned())
                            }
                            // Match any other times
                            _ => {
                                // If the logic time is bigger than the time given by the user
                                if logic[i] > u_users.time.into_inner() {
                                    availability::field::Time::create(logic[i].to_owned())
                                } else {
                                    availability::field::Time::create(u_users.time.into_inner())
                                }
                            }
                        },
                    },
                };
                // Increase the chosen count for the user
                lib::increase_chosen_user_count(
                    u_users.user_id.to_the_string().as_str(),
                    chosen_users,
                );
                list.push(new_employee);
            }
            None => list.push(Employee::no_user()),
        }
    }

    return list;
}

///* Calculate the schedule for a day of the week
pub fn calc_schedule_day(
    all_availability: &Vec<Availability>,
    all_users: &Vec<User>,
    schedule_logic: &Logic,
    day: &ScheduleDay,
    chosen_users: &mut HashMap<String, u8>,
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
                chosen_users,
            );
            roles.push(position)
        }
    }

    // Create the day schedule with the roles
    let the_day = DaySchedule {
        day: week::field::Days::create(day.to_owned()),
        employees: Employees {
            // Add the role from roles to the employees
            servers: roles.pop(),
            dishwashers: roles.pop(),
            bar: roles.pop(),
            kitchen: roles.pop(),
            griller: roles.pop(),
            manager: roles.pop(),
        },
    };
    return the_day;
}

///* Calculate the schedule for week of the staff
pub fn calc_schedule_week(
    all_availability: &Vec<Availability>,
    all_users: &Vec<User>,
    schedule_logic: &Logic,
    chosen_users: &mut HashMap<String, u8>,
) -> WeekSchedule {
    let mut week: Vec<DaySchedule> = Vec::new();

    for day in all::<ScheduleDay>() {
        let the_day = calc_schedule_day(
            &all_availability,
            &all_users,
            &schedule_logic,
            &day,
            chosen_users,
        );
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
