use std::str::FromStr;

enum Role {
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
    fn from_str(s: &str) -> Role {
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
    fn to_str(&self) -> String {
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
    pub admin: Bool,
    pub vast: Bool,
    pub suspended: Bool,
    pub min_days: u8,
    pub max_days: u8,
    pub role_primary: Role,
    pub role_secondary: Role,
}
impl User {
    fn new(
        id: u32,
        name: String,
        admin: Bool,
        vast: Bool,
        suspended: Bool,
        min_days: u8,
        max_days: u8,
        role_primary: Role,
        role_secondary: Role,
    ) -> User {
        let user = User {
            id,
            name,
            admin,
            vast,
            suspended,
            min_days,
            max_days,
            role_primary,
            role_secondary,
        };
    }
}

pub fn get_all_users() {
    let mut users_list = Vec::new();
    let kitchen1 = User::new(
        1,
        "Borris".to_string(),
        false,
        true,
        false,
        1,
        3,
        Role::Griller,
        Role::Kitchen,
    );
    let manager1 = User::new(
        2,
        "Greg".to_string(),
        false,
        true,
        false,
        1,
        3,
        Role::Management,
        Role::All,
    );
}
