// use scheduler::data;
// use scheduler::domain;
// use scheduler::service;
use scheduler::web;

use scheduler::data::DbWeekId;

// use serde::{Deserialize, Serialize};

fn main() {
    let plan1 = web::get_mock_data("1");

    let test = DbWeekId::new();

    println!("{:?}", test);

    // let year = Utc::now().year();
    // let week = Utc::now().week();
    // let week_id = format!("{}-{}", year, week);

    println!("{:?}", plan1);
}
