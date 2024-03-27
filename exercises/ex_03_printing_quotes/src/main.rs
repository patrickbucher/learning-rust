use std::io;

fn main() {
    let mut quote = String::new();
    println!("What is the quote?");
    io::stdin().read_line(&mut quote).expect("input needed");
    let quote = quote.trim();

    let mut author = String::new();
    println!("Who said it?");
    io::stdin().read_line(&mut author).expect("input needed");
    let author = author.trim();

    println!("{author} says, \"{quote}\"");
}
