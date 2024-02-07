use super::lib;

use crate::domain::availability::{self, Availability};
use crate::domain::user::User;
use crate::domain::{week, Role, ScheduleDay, ScheduleTime};
use crate::service::Logic;

use std::collections::HashMap;

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
    // Process Role
    let users: Vec<Option<Availability>> =
        lib::get_available_users(available_list, user_list, logic, role, day, chosen_users);

    let mut list_of_employees = Vec::new();

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
                list_of_employees.push(new_employee);
            }
            None => list_of_employees.push(Employee::no_user()),
        }
    }

    return list_of_employees;
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
