use crate::domain::availability::Availability;
use crate::domain::user::User;
use crate::domain::Role;
use rand::Rng;

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
