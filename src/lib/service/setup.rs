use crate::domain::{availability::AvailabilitySpot, user::User, Role, ScheduleDay};
use crate::service::{lib, Logic};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Roles {
    pub management: Vec<Option<AvailabilitySpot>>,
    pub griller: Vec<Option<AvailabilitySpot>>,
    pub kitchen: Vec<Option<AvailabilitySpot>>,
    pub bar: Vec<Option<AvailabilitySpot>>,
    pub dishwasher: Vec<Option<AvailabilitySpot>>,
    pub service: Vec<Option<AvailabilitySpot>>,
}
impl Roles {
    pub fn new(
        user_list: &Vec<User>,
        available_list: &Vec<AvailabilitySpot>,
        day: &ScheduleDay,
    ) -> Self {
        Self {
            management: sort_available_spots_current_day_on_role_bubble(
                user_list,
                available_list,
                day,
                &Role::Management,
            ),
            griller: sort_available_spots_current_day_on_role_bubble(
                user_list,
                available_list,
                day,
                &Role::Griller,
            ),
            kitchen: sort_available_spots_current_day_on_role_bubble(
                user_list,
                available_list,
                day,
                &Role::Kitchen,
            ),
            bar: sort_available_spots_current_day_on_role_bubble(
                user_list,
                available_list,
                day,
                &Role::Bar,
            ),
            dishwasher: sort_available_spots_current_day_on_role_bubble(
                user_list,
                available_list,
                day,
                &Role::Dishwasher,
            ),
            service: sort_available_spots_current_day_on_role_bubble(
                user_list,
                available_list,
                day,
                &Role::Service,
            ),
        }
    }
}

#[derive(Debug)]
pub struct Planning {
    pub day: ScheduleDay,
    pub roles: Roles,
}
impl Planning {
    pub fn new(
        user_list: &Vec<User>,
        available_list: &Vec<AvailabilitySpot>,
        the_day: &ScheduleDay,
    ) -> Self {
        Self {
            day: the_day.clone(),
            roles: Roles::new(user_list, available_list, &the_day),
        }
    }
}

#[derive(Debug)]
pub struct InfoMatrix {
    pub monday: Planning,
    pub tuesday: Planning,
    pub wednesday: Planning,
    pub thursday: Planning,
    pub friday: Planning,
    pub saturday: Planning,
    pub sunday: Planning,
}
impl InfoMatrix {
    pub fn new(all_users: &Vec<User>, all_availability: &Vec<AvailabilitySpot>) -> Self {
        Self {
            monday: Planning::new(all_users, all_availability, &ScheduleDay::Monday),
            tuesday: Planning::new(all_users, all_availability, &ScheduleDay::Tuesday),
            wednesday: Planning::new(all_users, all_availability, &ScheduleDay::Wednesday),
            thursday: Planning::new(all_users, all_availability, &ScheduleDay::Thursday),
            friday: Planning::new(all_users, all_availability, &ScheduleDay::Friday),
            saturday: Planning::new(all_users, all_availability, &ScheduleDay::Saturday),
            sunday: Planning::new(all_users, all_availability, &ScheduleDay::Sunday),
        }
    }
}

// Create info matrix will all available spots sorted on day and role
pub fn create_info_matrix(
    all_users: &Vec<User>,
    all_availability: &Vec<AvailabilitySpot>,
) -> InfoMatrix {
    let info_matrix = InfoMatrix::new(&all_users, &all_availability);

    return info_matrix;
}

// Sort all user list and return list of users based on role
pub fn filter_all_user_on_role(users: &Vec<User>, role: &Role) -> Vec<User> {
    let mut list: Vec<User> = Vec::new();
    for user in users.iter() {
        if user.role_primary.into_inner() == role || user.role_secondary.into_inner() == role {
            list.push(user.clone())
        } else if role != &Role::Management && user.role_secondary.into_inner() == &Role::All {
            list.push(user.clone())
        }
    }
    return list;
}

// Sort all available spots list and return list of available based on day
pub fn filter_available_spots_current_day(
    spots: Vec<AvailabilitySpot>,
    day: &ScheduleDay,
) -> Vec<Option<AvailabilitySpot>> {
    let mut list = Vec::new();

    for spot in spots {
        if spot.day.into_inner() == day {
            list.push(Some(spot))
        }
    }
    return list;
}

// Create a list of weekly chosen users
pub fn create_hashmap_tracker(filtered_user_list: &Vec<User>) -> HashMap<String, u8> {
    let mut chosen_users: HashMap<String, u8> = HashMap::new();
    for user in filtered_user_list {
        chosen_users.insert(user.id.to_the_string(), 0);
    }

    return chosen_users;
}

// Sort all available spots list and return list of available based on day and role
pub fn sort_available_spots_current_day_on_role(
    user_list: &Vec<User>,
    available_list: &Vec<AvailabilitySpot>,
    day: &ScheduleDay,
    role: &Role,
) -> Vec<Option<AvailabilitySpot>> {
    let mut final_list: Vec<Option<AvailabilitySpot>> = Vec::new();

    // User on role
    let filtered_user_list: Vec<User> = filter_all_user_on_role(&user_list, role);

    // Spots on day
    let filtered_available_list: Vec<Option<AvailabilitySpot>> =
        filter_available_spots_current_day(available_list.clone(), day);

    if filtered_available_list.is_empty() {
        return filtered_available_list;
    }

    for user in filtered_user_list {
        for available in filtered_available_list.iter() {
            if available.clone().unwrap().user_id.to_the_string() == user.id.to_the_string() {
                final_list.push(available.clone());
            }
        }
    }

    // Sort list on time supplied
    return final_list;
}

// Sort all available spots list and return list of available based on day and role
pub fn sort_available_spots_current_day_on_role_bubble(
    user_list: &Vec<User>,
    available_list: &Vec<AvailabilitySpot>,
    day: &ScheduleDay,
    role: &Role,
) -> Vec<Option<AvailabilitySpot>> {
    let mut final_list: Vec<Option<AvailabilitySpot>> = Vec::new();

    // User on role
    let filtered_user_list: Vec<User> = filter_all_user_on_role(&user_list, role);

    // Spots on day
    let filtered_available_list: Vec<Option<AvailabilitySpot>> =
        filter_available_spots_current_day(available_list.clone(), day);

    if filtered_available_list.is_empty() {
        return filtered_available_list;
    }

    for user in filtered_user_list {
        for available in filtered_available_list.iter() {
            if available.clone().unwrap().user_id.to_the_string() == user.id.to_the_string() {
                final_list.push(available.clone());
            }
        }
    }

    // Sort list on time supplied
    let sorted_final_list: Vec<Option<AvailabilitySpot>> =
        lib::bubble_sort_on_time_option(final_list);

    return sorted_final_list;
}

pub fn find_user(user_id: &str, user_list: &Vec<User>) -> Option<User> {
    let found_user: Option<User> = user_list
        .iter()
        .find(|user| user.id.to_the_string() == user_id)
        .cloned();
    return found_user;
}

pub fn increase_count_of_chosen_user(chosen: &mut HashMap<String, u8>, user_id: &str) {
    if let Some(count) = chosen.get_mut(user_id) {
        *count += 1;
    }
}

fn sort_availability_list_with_chosen(
    list: Vec<Option<AvailabilitySpot>>,
    chosen: &mut HashMap<String, u8>,
    user_list: &Vec<User>,
    logic: Logic,
) -> Vec<Option<AvailabilitySpot>> {
    let mut new_list: Vec<Option<AvailabilitySpot>> = Vec::new();
    let mut hold_list: Vec<Option<AvailabilitySpot>> = Vec::new();

    for availability_max_day in &list {
        if availability_max_day.is_some() {
            match availability_max_day {
                Some(availability) => {
                    let user_id_str = availability.user_id.to_the_string();
                    if let Some(count) = chosen.get(user_id_str.as_str()) {
                        if let Some(user) = find_user(user_id_str.as_str(), user_list) {
                            if *count < user.max_days.into_inner() {
                                new_list.push(Some(availability.clone()));
                                continue;
                            }
                        }
                    }
                }
                None => {}
            }
        } else {
            hold_list.push(availability_max_day.clone());
        }
    }

    new_list.extend(hold_list);
    return new_list;
}

#[cfg(test)]
mod test {
    use crate::data::DbId;
    use std::str::FromStr;

    use crate::domain::availability;
    use crate::domain::{
        availability::AvailabilitySpot, user::field, user::User, Role, ScheduleDay,
    };
    use crate::service::*;

    #[test]
    fn test_sort_available_spots_current_day_on_role() {
        let user1 = User::create_user(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            false,
            false,
            true,
            "4",
            "5",
            "griller",
            "kitchen",
        );
        let user2 = User::create_user(
            "a184afa7-1aeb-4cea-b8a8-278caa2dc36a",
            "Jane",
            false,
            false,
            true,
            "3",
            "4",
            "management",
            "Bar",
        );
        let user3 = User::create_user(
            "8ad23b27-707f-429c-b332-f504b2708185",
            "John",
            false,
            false,
            true,
            "2",
            "3",
            "management",
            "dishwasher",
        );
        let user4 = User::create_user(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Alice",
            false,
            false,
            true,
            "5",
            "5",
            "kitchen",
            "all",
        );

        // Eve
        let available1 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "monday",
            "13",
        );
        let available2 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "tuesday",
            "18",
        );
        let available3 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "wednesday",
            "17",
        );

        // Jane
        let available4 = AvailabilitySpot::create(
            "a184afa7-1aeb-4cea-b8a8-278caa2dc36a",
            "Jane",
            "monday",
            "15",
        );
        let available5 = AvailabilitySpot::create(
            "a184afa7-1aeb-4cea-b8a8-278caa2dc36a",
            "Jane",
            "tuesday",
            "17",
        );
        let available6 = AvailabilitySpot::create(
            "a184afa7-1aeb-4cea-b8a8-278caa2dc36a",
            "Jane",
            "thursday",
            "(17)",
        );

        // John
        let available7 = AvailabilitySpot::create(
            "8ad23b27-707f-429c-b332-f504b2708185",
            "John",
            "monday",
            "18",
        );
        let available8 = AvailabilitySpot::create(
            "8ad23b27-707f-429c-b332-f504b2708185",
            "John",
            "tuesday",
            "15",
        );
        let available9 = AvailabilitySpot::create(
            "8ad23b27-707f-429c-b332-f504b2708185",
            "John",
            "friday",
            "17(18)",
        );

        // Alice
        let available10 = AvailabilitySpot::create(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Alice",
            "monday",
            "18",
        );
        let available11 = AvailabilitySpot::create(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Alice",
            "tuesday",
            "15",
        );
        let available12 = AvailabilitySpot::create(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Alice",
            "saturday",
            "17(18)",
        );

        let user_list = vec![user1, user2, user3, user4];
        let available_list: Vec<AvailabilitySpot> = vec![
            available1.clone(),
            available2.clone(),
            available3.clone(),
            available4.clone(),
            available5.clone(),
            available6.clone(),
            available7.clone(),
            available8.clone(),
            available9.clone(),
            available10.clone(),
            available11.clone(),
            available12.clone(),
        ];
        let result = setup::sort_available_spots_current_day_on_role(
            &user_list,
            &available_list,
            &ScheduleDay::Monday,
            &Role::Griller,
        );
        let expected = vec![Some(available1.clone()), Some(available10.clone())];
        assert_eq!(
            result, expected,
            "Sorting failed on available spots current day(monday) for roles(griller)"
        );

        let result2 = setup::sort_available_spots_current_day_on_role(
            &user_list,
            &available_list,
            &ScheduleDay::Tuesday,
            &Role::Management,
        );
        let expected2 = vec![Some(available5), Some(available8)];
        assert_eq!(
            result2, expected2,
            "Sorting failed on available spots current day(tuesday) for roles(management)"
        );

        let result3 = setup::sort_available_spots_current_day_on_role(
            &user_list,
            &available_list,
            &ScheduleDay::Thursday,
            &Role::Bar,
        );
        let expected3: Vec<Option<AvailabilitySpot>> = vec![Some(available6)];
        assert_eq!(
            result3, expected3,
            "Sorting failed on available spots current day(thursday) for roles(bar)"
        );

        let result4 = setup::sort_available_spots_current_day_on_role(
            &user_list,
            &available_list,
            &ScheduleDay::Friday,
            &Role::Bar,
        );
        let expected4: Vec<Option<AvailabilitySpot>> = vec![];
        assert_eq!(
            result4, expected4,
            "Sorting failed on available spots current day(friday) for roles(bar)"
        );
    }

    #[test]
    fn test_sort_available_spots_current_day() {
        let available1 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "monday",
            "13",
        );

        let available2 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "tuesday",
            "18",
        );

        let available3 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "wednesday",
            "17",
        );
        let available4 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "thursday",
            "15",
        );

        let available5 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "friday",
            "17",
        );

        let available6 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "saturday",
            "(17)",
        );
        let available7 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "sunday",
            "18",
        );

        let available8 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "monday",
            "15",
        );

        let available9 = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "tuesday",
            "17(18)",
        );

        let list = vec![
            available1.clone(),
            available2.clone(),
            available3.clone(),
            available4.clone(),
            available5.clone(),
            available6.clone(),
            available7.clone(),
            available8.clone(),
            available9.clone(),
        ];

        let result = setup::filter_available_spots_current_day(list.clone(), &ScheduleDay::Monday);
        let expected = vec![Some(available1.clone()), Some(available8.clone())];

        let result2 =
            setup::filter_available_spots_current_day(list.clone(), &ScheduleDay::Tuesday);
        let expected2 = vec![Some(available2.clone()), Some(available9.clone())];

        let result3 =
            setup::filter_available_spots_current_day(list.clone(), &ScheduleDay::Saturday);
        let expected3: Vec<Option<AvailabilitySpot>> = vec![Some(available6.clone())];

        let list2: Vec<AvailabilitySpot> = Vec::new();
        let result4 = setup::filter_available_spots_current_day(list2, &ScheduleDay::Thursday);
        let expected4: Vec<Option<AvailabilitySpot>> = vec![];

        assert_eq!(result, expected, "Creating availability spots on monday");
        assert_eq!(result2, expected2, "Creating availability spots on tuesday");
        assert_eq!(
            result3, expected3,
            "Creating availability spots on saturday"
        );
        assert_eq!(result4, expected4, "No availability spots");
    }

    #[test]
    fn test_create_available_spots() {
        let result = AvailabilitySpot::create(
            "3da93583-e85f-4e21-b0b7-ade14abd72ae",
            "Eve",
            "monday",
            "15",
        );

        let expected = AvailabilitySpot {
            user_id: availability::field::AvailabilityId::new(
                DbId::from_str("3da93583-e85f-4e21-b0b7-ade14abd72ae").unwrap(),
            ),
            weekly_id: availability::field::WeeklyId::new(),
            name: availability::field::Name::new("Eve").unwrap(),
            day: availability::field::Days::new("monday"),
            time: availability::field::Time::new("15"),
        };
        assert_eq!(result, expected, "Creating availability spots");
    }

    #[test]
    fn test_create_user() {
        let id = field::UserID::default();
        let result = User {
            id: field::UserID::new(DbId::from_str(id.to_the_string().as_str()).unwrap()),
            name: field::Name::new("Eve").unwrap(),
            employee_id: field::EmployeeID::new(id.to_the_string().as_str()).unwrap(),
            admin: field::Admin::new(false),
            vast: field::Vast::new(false),
            active: field::Active::new(true),
            min_days: field::MinDays::new("4").unwrap(),
            max_days: field::MaxDays::new("5").unwrap(),
            role_primary: field::RolePrimary::new("griller").unwrap(),
            role_secondary: field::RoleSecondary::new("kitchen").unwrap(),
        };
        let expected = User::create_user(
            id.to_the_string().as_str(),
            "Eve",
            false,
            false,
            true,
            "4",
            "5",
            "griller",
            "kitchen",
        );

        assert_eq!(result, expected, "Creating user function is not equal");
    }

    #[test]
    fn test_sort_users1() {
        let id = field::UserID::default();
        let user1 = User::create_user(
            id.to_the_string().as_str(),
            "Eve",
            false,
            false,
            true,
            "4",
            "5",
            "griller",
            "kitchen",
        );
        let user2 = User::create_user(
            id.to_the_string().as_str(),
            "Jane",
            false,
            false,
            true,
            "3",
            "4",
            "service",
            "Bar",
        );
        let user3 = User::create_user(
            id.to_the_string().as_str(),
            "John",
            false,
            false,
            true,
            "2",
            "3",
            "management",
            "dishwasher",
        );
        let user4 = User::create_user(
            id.to_the_string().as_str(),
            "Alice",
            false,
            false,
            true,
            "5",
            "5",
            "kitchen",
            "all",
        );

        let users = vec![user1.clone(), user2.clone(), user3.clone(), user4.clone()];
        let role = Role::Service;
        let result = setup::filter_all_user_on_role(&users, &role);
        let expected = vec![user2.clone(), user4.clone()];

        assert_eq!(
            result, expected,
            "Expecting employees to sort based on role1"
        );
    }
    #[test]
    fn test_sort_users2() {
        let id = field::UserID::default();
        let user1 = User::create_user(
            id.to_the_string().as_str(),
            "Eve",
            false,
            false,
            true,
            "4",
            "5",
            "griller",
            "kitchen",
        );
        let user2 = User::create_user(
            id.to_the_string().as_str(),
            "Jane",
            false,
            false,
            true,
            "3",
            "4",
            "service",
            "Bar",
        );
        let user3 = User::create_user(
            id.to_the_string().as_str(),
            "John",
            false,
            false,
            true,
            "2",
            "3",
            "management",
            "dishwasher",
        );
        let user4 = User::create_user(
            id.to_the_string().as_str(),
            "Alice",
            false,
            false,
            true,
            "5",
            "5",
            "kitchen",
            "all",
        );

        let users = vec![user1.clone(), user2.clone(), user3.clone(), user4.clone()];
        let role = Role::Griller;
        let result = setup::filter_all_user_on_role(&users, &role);
        let expected = vec![user1.clone(), user4.clone()];

        assert_eq!(
            result, expected,
            "Expecting employees to sort based on role1"
        );
    }
    #[test]
    fn test_sort_users3() {
        let id = field::UserID::default();
        let user1 = User::create_user(
            id.to_the_string().as_str(),
            "Eve",
            false,
            false,
            true,
            "4",
            "5",
            "griller",
            "kitchen",
        );
        let user2 = User::create_user(
            id.to_the_string().as_str(),
            "Jane",
            false,
            false,
            true,
            "3",
            "4",
            "service",
            "Bar",
        );
        let user3 = User::create_user(
            id.to_the_string().as_str(),
            "John",
            false,
            false,
            true,
            "2",
            "3",
            "management",
            "dishwasher",
        );
        let user4 = User::create_user(
            id.to_the_string().as_str(),
            "Alice",
            false,
            false,
            true,
            "5",
            "5",
            "kitchen",
            "all",
        );
        let users = vec![user1.clone(), user2.clone(), user3.clone(), user4.clone()];
        let role = Role::Management;
        let result = setup::filter_all_user_on_role(&users, &role);
        let expected = vec![user3.clone()];

        assert_eq!(
            result, expected,
            "Expecting employees to sort based on role1"
        );
    }
}
