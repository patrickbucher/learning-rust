use chrono;
use chrono::Datelike;
use std::io;

fn main() {
    let mut age = String::new();
    println!("What is your current age?");
    io::stdin().read_line(&mut age).expect("input age");
    let age: i32 = age.trim().parse().expect("number");

    let mut retirement_age = String::new();
    println!("At what age would you like to retire?");
    io::stdin()
        .read_line(&mut retirement_age)
        .expect("input retirement year");
    let retirement_age: i32 = retirement_age.trim().parse().expect("number");

    let current_year = chrono::offset::Local::now().year();
    println!("{:?}", current_year);

    println!(
        "You have {} years left until you can retire.",
        retirement_age - age
    );
    println!(
        "It's {current_year}, so you can retire in {}.",
        current_year + (retirement_age - age)
    );
}
