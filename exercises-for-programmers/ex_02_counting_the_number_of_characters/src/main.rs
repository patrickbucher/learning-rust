use std::io;

fn main() {
    let mut line = String::new();
    println!("What is the input string?");
    io::stdin()
        .read_line(&mut line)
        .expect("no input was provided");
    let line = line.trim();
    println!("{line} has {} characters.", line.len());
}
