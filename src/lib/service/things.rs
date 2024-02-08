use crate::domain::{availability::AvailabilitySpot, user::User, Role, ScheduleDay};

struct Roles {
    management: Vec<Option<AvailabilitySpot>>,
    griller: Vec<Option<AvailabilitySpot>>,
    kitchen: Vec<Option<AvailabilitySpot>>,
    bar: Vec<Option<AvailabilitySpot>>,
    dishwasher: Vec<Option<AvailabilitySpot>>,
    service: Vec<Option<AvailabilitySpot>>,
}

struct Planning {
    day: ScheduleDay,
    roles: Roles,
}
struct InfoMatrix {
    monday: Planning,
    tuesday: Planning,
    wednesday: Planning,
    thursday: Planning,
    friday: Planning,
    saturday: Planning,
}

// Sort all user list and return list of users based on role
pub fn sort_all_user_on_role(users: &Vec<User>, role: &Role) -> Vec<User> {
    let mut list: Vec<User> = Vec::new();
    for user in users.iter() {
        if user.role_primary.into_inner() == role
            || user.role_secondary.into_inner() == role
            || user.role_secondary.into_inner() == &Role::All
        {
            list.push(user.clone())
        }
    }
    return list;
}

// Sort all available spots list and return list of available based on day
pub fn sort_available_spots_current_day(
    spots: Vec<AvailabilitySpot>,
    day: &ScheduleDay,
) -> Vec<Option<AvailabilitySpot>> {
    let mut list = Vec::new();

    for spot in spots.iter() {
        if spot.day.into_inner() == day {
            list.push(Some(spot.clone()))
        }
    }
    if list.is_empty() {
        list.push(None)
    }
    return list;
}

// fn sort_available_spots_current_day_on_role(
//     day_spot: Vec<Option<Availability>>,
//     role: &Role,
// ) -> Vec<Option<Availability>> {
// }

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
            "1", "Eve", false, false, true, "4", "5", "griller", "kitchen",
        );
        let user2 = User::create_user("2", "Jane", false, false, true, "3", "4", "service", "Bar");
        let user3 = User::create_user(
            "3",
            "John",
            false,
            false,
            true,
            "2",
            "3",
            "management",
            "dishwasher",
        );
        let user4 = User::create_user("5", "Alice", false, false, true, "5", "5", "kitchen", "all");

        let list2: Vec<AvailabilitySpot> = Vec::new();
        let result = things::sort_available_spots_current_day(list2, &ScheduleDay::Thursday);
        let expected = vec![None];

        assert_eq!(result, expected, "Creating availability spots on monday");
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

        let result = things::sort_available_spots_current_day(list.clone(), &ScheduleDay::Monday);
        let expected = vec![Some(available1.clone()), Some(available8.clone())];

        let result2 = things::sort_available_spots_current_day(list.clone(), &ScheduleDay::Tuesday);
        let expected2 = vec![Some(available2.clone()), Some(available9.clone())];

        let result3 =
            things::sort_available_spots_current_day(list.clone(), &ScheduleDay::Saturday);
        let expected3 = vec![Some(available6.clone())];

        let list2: Vec<AvailabilitySpot> = Vec::new();
        let result4 = things::sort_available_spots_current_day(list2, &ScheduleDay::Thursday);
        let expected4 = vec![None];

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
        let result = things::sort_all_user_on_role(&users, &role);
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
        let result = things::sort_all_user_on_role(&users, &role);
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
        let result = things::sort_all_user_on_role(&users, &role);
        let expected = vec![user3.clone(), user4.clone()];

        assert_eq!(
            result, expected,
            "Expecting employees to sort based on role1"
        );
    }
}
