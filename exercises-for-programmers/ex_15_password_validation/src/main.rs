use akshually::prompt_line;

const PASSWORD: &str = "topsecret";

fn main() {
    let password: String = prompt_line("What is the password? ").expect("password");
    if &password == PASSWORD {
        println!("Welcome!");
    } else {
        println!("I don't know you.");
    }
}
