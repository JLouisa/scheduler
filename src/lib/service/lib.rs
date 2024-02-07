use crate::domain::availability;
use crate::domain::availability::Availability;
use crate::domain::user::User;
use crate::domain::{Role, ScheduleDay};

use rand::Rng;
use std::collections::HashMap;

// Function to get a random number
pub fn get_random_number(num: usize) -> usize {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..num);

    return random_index;
}

// Function that iterates through a list of users and returns the user with the highest max_days
pub fn get_user_with_highest_max_days(list: &Vec<User>, role: &Role) -> Option<User> {
    let mut num = 0;
    let mut the_user: Option<User> = None;
    for user in list.iter() {
        if user.max_days.ref_into_inner() > num {
            num = user.max_days.ref_into_inner();
            the_user = Some(user.clone());
        }
    }
    return the_user;
}

// Function to sort the availability list
pub fn bubble_sort(arr: &mut Vec<Availability>) -> Vec<Availability> {
    let mut n = arr.len();
    let mut swapped = true;
    let mut arr = arr.clone();

    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i - 1].time.into_inner() > arr[i].time.into_inner() {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1; // Decrease n because the nth element is now in place
    }
    return arr;
}

// Function to increase the chosen user count
pub fn increase_chosen_user_count(user: &str, chosen_users_list: &mut HashMap<String, u8>) {
    let chosen_user = chosen_users_list.get_mut(user);
    match chosen_user {
        Some(value) => {
            *value += 1;
            println!(
                "The ID is {:?} and the times they were chosen is {:?}",
                user, value
            )
        }
        None => {
            // Handle the case where the key doesn't exist
            println!("Key {:?} not found in the HashMap", user);
        }
    }
}

// Function to find the lowest value in a HashMap
pub fn sort_lowest_to_highest_count(
    arr: Vec<Availability>,
    hash: &mut HashMap<String, u8>,
    vast_users: Vec<User>,
) -> Vec<Option<Availability>> {
    let mut new_arr: Vec<Option<Availability>> = Vec::new();
    let mut new_hash: HashMap<String, u8> = HashMap::new();

    // Iterate through the vast list to find the
    // user in the hashmap
    for user in arr.iter() {
        let found_vast_user = vast_users
            .iter()
            .find(|&u| &u.id.to_the_string() == &user.user_id.to_the_string());

        // Get the chosen count of the user
        let chosen_value_count = hash.get(user.user_id.to_the_string().as_str());

        // If found, this available user is a vast user
        match found_vast_user {
            Some(f_user) => {
                if let Some(value) = chosen_value_count {
                    if value < &f_user.max_days.ref_into_inner() {
                        new_arr.push(Some(user.clone()));
                    } else {
                        new_arr.push(None)
                    }
                }
            }
            None => {
                if let Some(&0) | None = chosen_value_count {
                    new_arr.push(Some(user.clone()));
                } else if let Some(value) = chosen_value_count {
                    new_hash.insert(user.user_id.to_the_string(), *value);
                } else {
                    unreachable!("Chosen value has no number");
                }
            }
        }
    }

    // Convert new_hash into a vector
    if !new_hash.is_empty() {
        let mut new_vec_from_hash: Vec<(String, u8)> = new_hash
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<Vec<(String, u8)>>();

        // Sort the new_hash
        new_vec_from_hash.sort_by(|a, b| a.0.cmp(&b.0));

        // Debug
        println!("This is the new hash: {:?}", new_vec_from_hash);

        // Iterate through the new_vec_from_hash and find the user
        // and push the user into the new_arr from low to high
        for user in new_vec_from_hash.iter() {
            let found_user = arr.iter().find(|&u| &u.user_id.to_the_string() == &user.0);
            match found_user {
                Some(f_user) => {
                    new_arr.push(Some(f_user.clone()));
                }
                None => new_arr.push(None),
            }
            // if found_user.is_some() {
            // }
        }
    }

    return new_arr;
}

//* Sorting lists
// 1. Get list of all users with Role from users in DB
pub fn sort_vast_users(user_list: &Vec<User>, role: &Role) -> Vec<User> {
    return user_list
        .clone()
        .iter()
        .cloned()
        .filter(|user| {
            &true == user.vast.into_inner()
                && (user.role_secondary.into_inner() == role
                    || user.role_primary.into_inner() == role
                    || user.role_secondary.into_inner() == &Role::All)
        })
        .collect::<Vec<User>>();
}

// 2. Get list of all available users on a given day from available users
pub fn sort_available_users(
    list: &Vec<availability::Availability>,
    day: &ScheduleDay,
) -> Vec<availability::Availability> {
    return list
        .iter()
        .cloned()
        .filter(|availability| availability.day.into_inner() == day)
        .collect::<Vec<availability::Availability>>();
}

// 3. Get list of available roles on a given day from available users on that day
pub fn sort_available_users_on_role(
    list_vast_users: &Vec<User>,
    day_available_users: &Vec<availability::Availability>,
) -> Vec<availability::Availability> {
    let mut new_vec = Vec::new();

    for manager in list_vast_users.clone() {
        for user in day_available_users.clone() {
            if user.user_id.to_the_string() == manager.id.to_the_string() {
                new_vec.push((user).clone())
            }
        }
    }
    new_vec
}
