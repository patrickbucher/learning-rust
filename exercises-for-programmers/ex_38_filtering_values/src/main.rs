use akshually::io::prompt_line;

fn main() {
    let input: String = prompt_line("Enter a list of numbers, separated by spaces: ").unwrap();
    let tokens: Vec<i32> = input
        .split(" ")
        .map(|t| t.parse::<i32>())
        .map(|x| x.unwrap_or(-1))
        .filter(|x| x % 2 == 0)
        .collect();
    println!(
        "{}",
        tokens
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
