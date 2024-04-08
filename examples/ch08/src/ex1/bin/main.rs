fn main() {
    let numbers: Vec<u32> = vec![0, 1, 7, 8, 9];
    println!("median of {numbers:?} is {:?}", median(&numbers));
}

fn median(values_sorted: &Vec<u32>) -> Option<u32> {
    if values_sorted.len() % 2 == 0 {
        None
    } else {
        Some(values_sorted[values_sorted.len() / 2])
    }
}
