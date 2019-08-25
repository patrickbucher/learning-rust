extern crate mememo;

fn main() {
    let numbers = vec![1, 2, 3];
    let mean = mememo::mean(&numbers);
    println!("mean of {:?} is {}", numbers, mean);
}
