use super::setup::InfoMatrix;

use crate::data::DbId;
use crate::domain::availability::{self, AvailabilitySpot};
use crate::domain::user::User;
use crate::domain::{week, Role, ScheduleTime};

use crate::service::setup;
use crate::service::Logic;

use std::collections::HashMap;
use std::str::FromStr;

// use enum_iterator::all;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Employee {
    id: availability::field::AvailabilityId,
    name: availability::field::Name,
    time_available: availability::field::Time,
    time_given: availability::field::Time,
}
impl Employee {
    pub fn create(
        id: &str,
        name: &str,
        time_available: ScheduleTime,
        time_given: ScheduleTime,
    ) -> Self {
        Self {
            id: availability::field::AvailabilityId::new(DbId::from_str(id).unwrap()),
            name: availability::field::Name::new(name).unwrap(),
            time_available: availability::field::Time::create(time_available),
            time_given: availability::field::Time::create(time_given),
        }
    }
    pub fn no_user() -> Self {
        Self {
            id: availability::field::AvailabilityId::default(),
            name: availability::field::Name::new("No User").expect("cannot parse No User name"),
            time_available: availability::field::Time::create(ScheduleTime::None),
            time_given: availability::field::Time::create(ScheduleTime::None),
        }
    }
    pub fn new(available: &Option<AvailabilitySpot>, the_time: ScheduleTime) -> Self {
        if available.is_some() {
            Self::create(
                available
                    .clone()
                    .expect("unwrapping available ID in Employee impl.")
                    .user_id
                    .to_the_string()
                    .as_str(),
                available
                    .clone()
                    .expect("unwrapping available name in Employee impl.")
                    .name
                    .as_str(),
                available
                    .clone()
                    .expect("unwrapping available time in Employee impl.")
                    .time
                    .into_inner(),
                the_time,
            )
        } else {
            Self::no_user()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Employees {
    pub manager: Vec<Employee>,
    pub griller: Vec<Employee>,
    pub kitchen: Vec<Employee>,
    pub bar: Vec<Employee>,
    pub dishwashers: Vec<Employee>,
    pub servers: Vec<Employee>,
}

#[derive(Debug, Clone)]
pub struct DaySchedule {
    pub day: week::field::Days,
    pub employees: Employees,
}

#[derive(Debug)]
pub struct WeekSchedule {
    pub weekly_id: week::field::WeeklyId,
    pub monday: DaySchedule,
    pub tuesday: DaySchedule,
    pub wednesday: DaySchedule,
    pub thursday: DaySchedule,
    pub friday: DaySchedule,
    pub saturday: DaySchedule,
    pub sunday: DaySchedule,
}

///* Calculate the schedule for week of the staff
pub fn calc_schedule_week(
    sorted_week: InfoMatrix,
    schedule_logic: Logic,
    chosen_users: &mut HashMap<String, u8>,
    all_users: Vec<User>,
) {
    let managers_list = calc_schedule_week_manager(
        &sorted_week.monday.roles.management,
        &schedule_logic,
        chosen_users,
        &all_users,
    );
    for manager in managers_list.iter() {
        if manager.is_some() {
            println!(
                "Managers List: {:?}",
                manager.clone().expect("Unable to unwrap manager")
            );
        } else {
            println!("Managers List: None");
        }
    }
}

pub fn calc_schedule_week_manager(
    manager_list: &Vec<Option<AvailabilitySpot>>,
    schedule_logic: &Logic,
    chosen_users: &mut HashMap<String, u8>,
    all_users: &Vec<User>,
) -> Vec<Option<AvailabilitySpot>> {
    let mut new_manager_list = Vec::new();
    let mut hold_list = Vec::new();
    let logic_length_manager_list = schedule_logic.manager.len();
    let users_by_role = setup::filter_all_user_on_role(all_users, &Role::Management);

    // Manager list getting from filter available list
    for available_spot_option in manager_list {
        if available_spot_option.is_some() {
            let available = available_spot_option.clone().unwrap();
            // let chosen = chosen_users.get(&available.user_id.to_the_string());
            let found_user = found_user_fn(&users_by_role, &available_spot_option.clone().unwrap());
            // Process if the user is found in the list of users
            if found_user.is_some() {
                increase_chosen_user_count(
                    // &chosen,
                    &found_user.unwrap(),
                    &mut new_manager_list,
                    chosen_users,
                    &available,
                );
            }
        } else {
            hold_list.push(available_spot_option.clone());
        }

        if new_manager_list.len() >= logic_length_manager_list {
            break;
        }
    }

    new_manager_list.extend_from_slice(&hold_list);
    new_manager_list
}

fn found_user_fn(users_by_role: &Vec<User>, available: &AvailabilitySpot) -> Option<User> {
    let found_user = users_by_role
        .iter()
        .cloned()
        .find(|x| x.id.to_the_string() == available.user_id.to_the_string());
    return found_user;
}

fn increase_chosen_user_count(
    // chosen: &Option<&u8>,
    found_user: &User,
    new_manager_list: &mut Vec<Option<AvailabilitySpot>>,
    chosen_users: &mut HashMap<String, u8>,
    available: &AvailabilitySpot,
) {
    let chosen = chosen_users.get(&available.user_id.to_the_string());

    if let Some(chosen_num) = chosen {
        if *chosen_num < found_user.max_days.ref_into_inner()
            && found_user.vast.into_inner() == &true
        {
            new_manager_list.push(Some(available.clone()));
            chosen_users
                .entry(available.user_id.to_the_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::availability::AvailabilitySpot;
    use crate::domain::ScheduleTime;
    use crate::service::{scheduler, Logic};

    use std::collections::HashMap;

    #[test]
    #[ignore]
    fn test_calc_schedule_week_manager() {
        // 3. Get the schedule logic
        let schedule_logic = Logic {
            manager: vec![ScheduleTime::StartAtThree],
            griller: vec![ScheduleTime::StartAtThree],
            kitchen: vec![ScheduleTime::StartAtThree, ScheduleTime::StartAtSix],
            bar: vec![ScheduleTime::StartAtSix],
            dishwashers: vec![ScheduleTime::StartAtSix],
            servers: vec![
                ScheduleTime::StartAtThree,
                ScheduleTime::StartAtFive,
                ScheduleTime::StartAtSix,
                ScheduleTime::OnCallAtSix,
            ],
        };

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
            "2",
            "3",
            "service",
            "Bar",
        );
        let user3 = User::create_user(
            "8ad23b27-707f-429c-b332-f504b2708185",
            "John",
            false,
            true,
            true,
            "5",
            "5",
            "management",
            "dishwasher",
        );
        let user4 = User::create_user(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Alice",
            false,
            false,
            true,
            "4",
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

        let all_users = vec![user1, user2, user3, user4];
        let available = vec![Some(available4.clone()), Some(available8.clone())];
        // let manager_list = vec![user3];

        // 1.1 Create a list of weekly chosen users
        let mut chosen_users: HashMap<String, u8> = setup::create_hashmap_tracker(&all_users);

        let role = Role::Management;

        let result = scheduler::calc_schedule_week_manager(
            &available,
            &schedule_logic,
            &mut chosen_users,
            &all_users,
        );
        // let expected = vec![user2.clone(), user4.clone()];

        // assert_eq!(
        //     result, expected,
        //     "Expecting employees to sort based on role1"
        // );
    }

    #[test]
    fn test_find_user() {
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
            "2",
            "3",
            "service",
            "Bar",
        );
        let user3 = User::create_user(
            "8ad23b27-707f-429c-b332-f504b2708185",
            "John",
            false,
            true,
            true,
            "5",
            "5",
            "management",
            "dishwasher",
        );
        let user4 = User::create_user(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Adam",
            false,
            false,
            true,
            "4",
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

        // Adam
        let available10 = AvailabilitySpot::create(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Adam",
            "monday",
            "18",
        );

        let available11 = AvailabilitySpot::create(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Adam",
            "tuesday",
            "15",
        );

        let available12 = AvailabilitySpot::create(
            "5b3e2a19-fd6d-478e-a69c-3c679449f34a",
            "Adam",
            "saturday",
            "17(18)",
        );

        let all_users = vec![user1.clone(), user2.clone(), user3.clone(), user4.clone()];
        let all_available = vec![
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

        let result = scheduler::found_user_fn(&all_users, &available1);
        let expected = Some(user1);

        assert_eq!(
            result, expected,
            "Expecting Eve to be found in the list of users"
        );
        let result2 = scheduler::found_user_fn(&all_users, &available9);
        let expected2 = Some(user3);

        assert_eq!(
            result2, expected2,
            "Expecting John to be found in the list of users"
        );
        let result3 = scheduler::found_user_fn(&all_users, &available12);
        let expected3 = Some(user4);

        assert_eq!(
            result3, expected3,
            "Expecting Adam to be found in the list of users"
        );
    }
}
