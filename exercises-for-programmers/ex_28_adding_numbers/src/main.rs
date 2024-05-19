use akshually::io::prompt_line;

fn main() {
    let mut total: i32 = 0;
    for _i in 0..5 {
        let number: i32 = prompt_line("Enter a number: ").unwrap();
        total += number;
    }
    println!("The total is {total}.");
}
