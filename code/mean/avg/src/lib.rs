extern crate sum;

/// Calculates the average of the given vector and returns it.
///
/// # Example
///
/// ```
/// let numbers = vec![1, 2, 3, 4];
/// assert_eq!(2.5, avg::avg(numbers));
/// ```
pub fn avg(numbers: Vec<i32>) -> f64 {
    let n = numbers.len();
    let sum = sum::sum(numbers);
    let average: f64 = (sum as f64) / (n as f64);
    average
}
