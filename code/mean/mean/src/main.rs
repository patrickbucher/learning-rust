extern crate avg;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("numbers: {:?}", numbers);
    let average = avg::avg(numbers);
    println!("mean thereof: {}", average);
}
