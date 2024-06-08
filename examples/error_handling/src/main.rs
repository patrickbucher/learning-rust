fn main() {
    let inputs = ["123", "abc", "17", "", "1"];
    let sum: usize = inputs
        .iter()
        .map(|s| s.parse::<usize>())
        .map(|x| x.unwrap_or(0))
        .sum();
    println!("The sum of {:?} is {sum}.", inputs);
}
