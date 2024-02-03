use crate::domain::user::User;
use rand::Rng;

pub fn get_random_number(num: usize) -> usize {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..num);

    return random_index;
}

// Function that iterates through a list of users and returns the user with the highest max_days
pub fn get_user_with_highest_max_days(list: &Vec<User>) -> User {
    let mut num = 0;
    let mut the_user = None;
    for user in list.iter() {
        if user.max_days.ref_into_inner() > num {
            num = user.max_days.ref_into_inner();
            the_user = Some(user.clone());
        }
    }
    return the_user.expect("No user found");
}
