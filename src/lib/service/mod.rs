use crate::domain::availability::Availability;
use crate::domain::user;
use crate::domain::week::field;

use crate::web;

struct Employee(field::Name);

struct Employees {
    manager: Employee,
    griller: Employee,
    bar: Employee,
    servers: Vec<Employee>,
    dishwashers: Vec<Employee>,
}

struct MondaySchedule {
    weekly_id: String,
    day: field::Days,
    employees: Employees,
}

pub fn calc_monday_schedule() {
    let available1 = web::get_mock_data("1").expect("Failed to get mock data");
    let available2 = web::get_mock_data("2").expect("Failed to get mock data");
    let available3 = web::get_mock_data("3").expect("Failed to get mock data");
    let available4 = web::get_mock_data("4").expect("Failed to get mock data");
    let available5 = web::get_mock_data("5").expect("Failed to get mock data");
    let available_user_list = vec![available1, available2, available3, available4, available5];

    for user in available_user_list {
        println!("user: {:?}", user);
    }
}
