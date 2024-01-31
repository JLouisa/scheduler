use scheduler::database;
use scheduler::service;
use scheduler::web;

use serde::{Deserialize, Serialize};

fn main() {
    let plan1 = web::get_mock_data("1");

    println!("{:?}", plan1);
}
