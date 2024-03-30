use std::io;

fn main() {
    let mut first = String::new();
    println!("What is the first number?");
    io::stdin().read_line(&mut first).expect("first number");
    let first: i32 = first.trim().parse().expect("numeric input");

    let mut second = String::new();
    println!("What is the second number?");
    io::stdin().read_line(&mut second).expect("second number");
    let second: i32 = second.trim().parse().expect("numeric input");

    println!(
        "{} + {} = {}\n{} - {} = {}\n{} * {} = {}\n{} / {} = {}",
        first,
        second,
        first + second,
        first,
        second,
        first - second,
        first,
        second,
        first * second,
        first,
        second,
        first / second
    );
}
