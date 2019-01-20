use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 2, 3, 1, 3, 2, 1, 2, 2, 3, 4, 3, 2, 5];
    println!("mean: {}", mean(&numbers));
    match median(&numbers) {
        Median::MiddleSingle(i) => println!("median (single): {}", i),
        Median::MiddleTwoMean(f) => println!("median (of two): {}", f),
    };
    println!("mode: {}", mode(&numbers));
}

fn mean(numbers: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in numbers {
        sum += i;
    }
    sum as f64 / numbers.len() as f64
}

enum Median {
    MiddleSingle(i32),
    MiddleTwoMean(f64),
}

fn median(numbers: &Vec<i32>) -> Median {
    let i = numbers.len() / 2;
    if numbers.len() % 2 == 0 && numbers.len() >= 2 {
        // even: mean of middle two
        Median::MiddleTwoMean((numbers[i] + numbers[i + 1]) as f64 / 2.0)
    } else {
        Median::MiddleSingle(numbers[i])
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    let mut max_key: i32 = 0;
    let mut max_val: i32 = 0;
    for i in numbers {
        let count = counter.entry(*i).or_insert(0);
        *count += 1;
    }
    for (key, val) in counter {
        if val > max_val {
            max_key = key;
            max_val = val;
        }
    }
    max_key
}
