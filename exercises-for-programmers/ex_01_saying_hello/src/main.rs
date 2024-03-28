use std::io;

fn main() {
    println!("What is your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("enter your name");
    let name = input.trim();
    println!("Hello, {name}, nice to meet you!");
    // TODO: separate string concatenation and output (constraint)
}
