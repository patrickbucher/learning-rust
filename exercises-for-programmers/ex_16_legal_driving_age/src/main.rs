use akshually::prompt_line;

const LEGAL_DRIVING_AGE: u8 = 16;

fn main() {
    let age: u8 = prompt_line("What is your age? ").expect("age");
    if age >= LEGAL_DRIVING_AGE {
        println!("You are old enough to legally drive.");
    } else {
        println!("You are not old enough to legally drive.");
    }
}
