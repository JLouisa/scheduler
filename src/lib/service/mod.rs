use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database;
use crate::web;

#[derive(Debug, Serialize, Deserialize)]
pub enum Time {
    StartAtOne,
    StartAtThree,
    StartAtFive,
    StartAtSix,
    FromOneToFive,
    FromThreeToFive,
    Custom(String),
    OnCall(String),
}
impl Time {
    pub fn to_num(&self) -> String {
        match self {
            Time::StartAtOne => "13".to_owned(),
            Time::StartAtThree => "15".to_owned(),
            Time::StartAtFive => "17".to_owned(),
            Time::StartAtSix => "18".to_owned(),
            Time::FromOneToFive => "13-17".to_owned(),
            Time::FromThreeToFive => "15-17".to_owned(),
            Time::Custom(n) => n.to_owned(),
            Time::OnCall(n) => n.to_owned(),
        }
    }
    pub fn from_str(s: &str) -> Time {
        match s {
            "13" => Time::StartAtOne,
            "15" => Time::StartAtThree,
            "17" => Time::StartAtFive,
            "18" => Time::StartAtSix,
            "13-17" => Time::FromOneToFive,
            "15-17" => Time::FromThreeToFive,
            _ => Time::Custom(s.to_owned()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    min_days: u8,
    max_days: u8,
    role_primary: database::Role,
    role_secondary: database::Role,
    chosen: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAvailability {
    name: String,
    time: Time,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availabilities {
    monday: Vec<UserAvailability>,
    tuesday: Vec<UserAvailability>,
    wednesday: Vec<UserAvailability>,
    thursday: Vec<UserAvailability>,
    friday: Vec<UserAvailability>,
    saturday: Vec<UserAvailability>,
    sunday: Vec<UserAvailability>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockData {
    admin: String,
    date: DateTime<Utc>,
    availabilities: Availabilities,
}
impl From<web::MockData> for MockData {
    fn from(old: web::MockData) -> MockData {
        let mut new_week = MockData {
            admin: old.admin,
            date: old.date,
            availabilities: Availabilities {
                monday: Vec::new(),
                tuesday: Vec::new(),
                wednesday: Vec::new(),
                thursday: Vec::new(),
                friday: Vec::new(),
                saturday: Vec::new(),
                sunday: Vec::new(),
            },
        };

        for user in old.availabilities.monday.iter() {
            new_week.availabilities.monday.push(UserAvailability {
                name: user.name.to_owned(),
                time: Time::from_str(user.time.to_string().as_str()),
            })
        }

        for user in old.availabilities.tuesday {
            new_week.availabilities.tuesday.push(UserAvailability {
                name: user.name.to_owned(),
                time: Time::from_str(user.time.to_string().as_str()),
            })
        }

        for user in old.availabilities.wednesday {
            new_week.availabilities.wednesday.push(UserAvailability {
                name: user.name.to_owned(),
                time: Time::from_str(user.time.to_string().as_str()),
            })
        }

        for user in old.availabilities.thursday {
            new_week.availabilities.thursday.push(UserAvailability {
                name: user.name.to_owned(),
                time: Time::from_str(user.time.to_string().as_str()),
            })
        }

        for user in old.availabilities.friday {
            new_week.availabilities.thursday.push(UserAvailability {
                name: user.name.to_owned(),
                time: Time::from_str(user.time.to_string().as_str()),
            })
        }

        for user in old.availabilities.saturday {
            new_week.availabilities.thursday.push(UserAvailability {
                name: user.name.to_owned(),
                time: Time::from_str(user.time.to_string().as_str()),
            })
        }

        for user in old.availabilities.sunday {
            new_week.availabilities.thursday.push(UserAvailability {
                name: user.name.to_owned(),
                time: Time::from_str(user.time.to_string().as_str()),
            })
        }

        return new_week;
    }
}

// pub fn get_converted_users() -> MockData {
//     let availability_week = web::get_mock_data();
//     let availability_week = MockData::from(availability_week);

//     println!("{:?}", availability_week);

//     return availability_week;
// }

pub struct TheDay {
    day: database::Days,
    user_list: Vec<User>,
}

pub struct Week {
    monday: TheDay,
    tuesday: TheDay,
    wednesday: TheDay,
    thursday: TheDay,
    friday: TheDay,
    saturday: TheDay,
    sunday: TheDay,
}

fn process() {
    // This is the structure to be saved in the database as weekly schedule
    let mut new_week = Week {
        monday: TheDay {
            day: database::Days::Monday,
            user_list: Vec::new(),
        },
        tuesday: TheDay {
            day: database::Days::Tuesday,
            user_list: Vec::new(),
        },
        wednesday: TheDay {
            day: database::Days::Wednesday,
            user_list: Vec::new(),
        },
        thursday: TheDay {
            day: database::Days::Thursday,
            user_list: Vec::new(),
        },
        friday: TheDay {
            day: database::Days::Friday,
            user_list: Vec::new(),
        },
        saturday: TheDay {
            day: database::Days::Saturday,
            user_list: Vec::new(),
        },
        sunday: TheDay {
            day: database::Days::Sunday,
            user_list: Vec::new(),
        },
    };

    // let users_availability_data = service::get_converted_users();
    // let user_list = database::get_all_users();

    // let availability_week: Availabilities =
    //     serde_json::from_str(&availability_week).expect("Failed to parse mock data");

    // let user_list: Vec<database::User> = database::get_all_users();
    // let mut converted_user_list: Vec<User> = Vec::new();

    // for user in user_list.iter() {
    //     let converted_user = User {
    //         name: user.name.clone(),
    //         min_days: user.min_days,
    //         max_days: user.max_days,
    //         role_primary: user.role_primary.clone(),
    //         role_secondary: user.role_secondary.clone(),
    //         chosen: 0,
    //     };
    //     converted_user_list.push(converted_user);
    // }
}
