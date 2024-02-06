use crate::domain::availability::Availability;
use crate::domain::user::{self, User};
use crate::domain::Role;

use rand::Rng;
use std::collections::HashMap;
use std::iter;

// Function to get a random number
pub fn get_random_number(num: usize) -> usize {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..num);

    return random_index;
}

// Function that iterates through a list of users and returns the user with the highest max_days
pub fn get_user_with_highest_max_days(list: &Vec<User>, role: &Role) -> User {
    let mut num = 0;
    let mut the_user = None;
    for user in list.iter() {
        if user.max_days.ref_into_inner() > num {
            num = user.max_days.ref_into_inner();
            the_user = Some(user.clone());
        }
    }
    return the_user.expect(format!("No user found with role: {:?}", role).as_str());
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
) -> Vec<Availability> {
    let mut new_arr: Vec<Availability> = Vec::new();
    let mut new_hash: HashMap<String, u8> = HashMap::new();

    // Iterate through the HashMap and find the lowest value based on availability list
    for user in arr.iter() {
        let found_vast_user = vast_users
            .iter()
            .find(|&u| &u.id.to_the_string() == &user.user_id.to_the_string());
        let value = hash.get(user.user_id.to_the_string().as_str());

        //If found, this available user is a vast user
        if found_vast_user.is_some() {
            if value.unwrap() < &found_vast_user.unwrap().max_days.ref_into_inner() {
                new_arr.push(user.clone());
            }
        } else if value.unwrap() == &0 {
            new_arr.push(user.clone());
        } else {
            new_hash.insert(user.user_id.to_the_string(), *value.unwrap());
        }
    }

    // Convert new_hash into a vector
    let mut new_hash_vec: Vec<(String, u8)> = new_hash
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<(String, u8)>>();

    // Sort the new_hash
    new_hash_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // Iterate through the new_hash_vec and find the user
    // and push the user into the new_arr from low to high
    for user in new_hash_vec.iter() {
        let found_user = arr.iter().find(|&u| &u.user_id.to_the_string() == &user.0);
        if found_user.is_some() {
            new_arr.push(found_user.unwrap().clone());
        }
    }

    return new_arr;
}
