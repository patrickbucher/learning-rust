use akshually;

fn main() {
    if let Some(input) = akshually::prompt_line::<String>("What is your name? ") {
        let name = input.trim();
        let output = format!("Hello, {name}, nice to meet you!");
        println!("{output}");
    }
}
