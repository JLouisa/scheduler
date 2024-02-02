use crate::domain::availability::Availability;
use crate::domain::user;
use crate::domain::week::field;

use crate::data::db;
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

    for user in available_user_list.iter() {
        println!("user from available:");
        println!(" {:?}", user);
        println!("",);
    }

    let user1 = db::get_mock_user("1").expect("Failed to get mock user");
    let user2 = db::get_mock_user("2").expect("Failed to get mock user");
    let user3 = db::get_mock_user("3").expect("Failed to get mock user");
    let user4 = db::get_mock_user("4").expect("Failed to get mock user");
    let user5 = db::get_mock_user("5").expect("Failed to get mock user");
    let user_list = vec![user1, user2, user3, user4, user5];

    for user in user_list.iter() {
        println!("user from DB:");
        println!(" {:?}", user);
        println!("",);
    }
}
