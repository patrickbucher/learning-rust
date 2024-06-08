use itertools::Itertools;

fn main() {
    let inputs = ["123", "abc", "17", "", "1"];
    let (numbers, errors): (Vec<usize>, Vec<_>) =
        inputs.iter().map(|s| s.parse::<usize>()).partition_result();
    println!("numbers: {:?}", numbers);
    println!("errors: {:?}", errors);
}
