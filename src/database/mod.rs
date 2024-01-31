use serde::{Deserialize, Serialize};
// use std::str::FromStr;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Griller,
    Kitchen,
    Bar,
    Service,
    Management,
    Dishwasher,
    None,
    All,
}
impl Role {
    pub fn from_str(s: &str) -> Role {
        match s {
            "Griller" => Role::Griller,
            "Kitchen" => Role::Kitchen,
            "Bar" => Role::Bar,
            "Service" => Role::Service,
            "Management" => Role::Management,
            "Dishwasher" => Role::Dishwasher,
            "None" => Role::None,
            "All" => Role::All,
            _ => Role::None,
        }
    }
    pub fn to_str(&self) -> String {
        match *self {
            Role::Griller => "Griller".to_string(),
            Role::Kitchen => "Kitchen".to_string(),
            Role::Bar => "Bar".to_string(),
            Role::Service => "Service".to_string(),
            Role::Management => "Management".to_string(),
            Role::Dishwasher => "Dishwasher".to_string(),
            Role::None => "None".to_string(),
            Role::All => "All".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub week_id: String,
    pub admin: bool,
    pub vast: bool,
    pub active: bool,
    pub min_days: u8,
    pub max_days: u8,
    pub role_primary: Role,
    pub role_secondary: Role,
}
impl User {
    fn new(
        id: u32,
        name: String,
        week_id: String,
        admin: bool,
        vast: bool,
        active: bool,
        min_days: u8,
        max_days: u8,
        role_primary: Role,
        role_secondary: Role,
    ) -> Self {
        Self {
            id,
            name,
            week_id,
            admin,
            vast,
            active,
            min_days,
            max_days,
            role_primary,
            role_secondary,
        }
    }
}

#[derive(Debug)]
pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// pub fn get_all_users() -> Vec<User> {
//     let mut users_list = Vec::new();
//     let manager1 = User::new(
//         1,
//         "Greg".to_string(),
//         false,
//         true,
//         false,
//         5,
//         5,
//         Role::Management,
//         Role::All,
//     );
//     let griller1 = User::new(
//         2,
//         "Boris".to_string(),
//         false,
//         true,
//         false,
//         5,
//         5,
//         Role::Griller,
//         Role::Kitchen,
//     );
//     let kitchen1 = User::new(
//         3,
//         "Anne".to_string(),
//         false,
//         false,
//         false,
//         5,
//         5,
//         Role::Kitchen,
//         Role::None,
//     );
//     let bar1 = User::new(
//         4,
//         "Jonathan".to_string(),
//         false,
//         false,
//         false,
//         4,
//         5,
//         Role::Service,
//         Role::Bar,
//     );
//     let service1 = User::new(
//         5,
//         "Charmaine".to_string(),
//         false,
//         false,
//         false,
//         3,
//         4,
//         Role::Service,
//         Role::Kitchen,
//     );
//     let dishwasher = User::new(
//         6,
//         "Kimberly".to_string(),
//         false,
//         false,
//         false,
//         3,
//         4,
//         Role::Dishwasher,
//         Role::None,
//     );
//     users_list.push(manager1);
//     users_list.push(griller1);
//     users_list.push(kitchen1);
//     users_list.push(bar1);
//     users_list.push(service1);
//     users_list.push(dishwasher);
//     return users_list;
// }
