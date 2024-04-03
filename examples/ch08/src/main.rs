use std::collections::HashMap;

fn main() {
    // Vectors
    let mut fibs: Vec<u32> = vec![1, 1];
    fibs.push(fibs[0] + fibs[1]);
    fibs.push(fibs[fibs.len() - 2] + fibs[fibs.len() - 1]);
    println!("{fibs:?}");
    match fibs.get(3) {
        Some(number) => println!("fibs[3]: {number}"),
        None => println!("fib[3]: undefined"),
    }
    for f in fibs {
        println!("{}", f);
    }

    // String
    let mut greeting = String::from("Здравсивуйте");
    greeting.push(' ');
    greeting.push_str("мир");
    println!("{greeting}");
    for c in greeting.chars() {
        println!("{}", c);
    }

    // HashMaps
    let mut freqs = HashMap::new();
    let text = String::from("alda lass das kalkfass da");
    for c in text.chars() {
        let mut freq = freqs.entry(c).or_insert(0);
        *freq += 1;
    }
    println!("{freqs:?}");
}
