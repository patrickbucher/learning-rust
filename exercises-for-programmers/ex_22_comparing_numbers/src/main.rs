use akshually::io::prompt_line;
use std::cmp;
use std::process;

fn main() {
    let a: i32 = prompt_line("Enter the first number: ").unwrap();
    let b: i32 = prompt_line("Enter the second number: ").unwrap();
    let c: i32 = prompt_line("Enter the third number: ").unwrap();

    if a == b || b == c || a == c {
        eprintln!("the numbers are not different");
        process::exit(1);
    }

    let numbers = vec![a, b, c];
    let largest = find_largest(numbers);
    println!("the largest number is {}", largest.unwrap());
}

fn find_largest(numbers: Vec<i32>) -> Option<i32> {
    let mut max: Option<i32> = None;
    for i in numbers {
        match max {
            Some(j) => {
                if i > j {
                    max = Some(i);
                }
            }
            None => {
                max = Some(i);
            }
        }
    }
    max
}
