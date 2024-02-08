use scheduler::service;

fn main() {
    let new_week = service::schedule_setup();
    println!("Week Schedule: {:?}", new_week);
}
