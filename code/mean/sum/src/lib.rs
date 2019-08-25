/// Sums up the given vector and returns the sum.
///
/// # Example
///
/// ```
/// let numbers = vec![1, 2, 3, 4];
/// assert_eq!(10, sum::sum(numbers));
/// ```
pub fn sum(numbers: Vec<i32>) -> i32 {
    let mut result = 0;
    for n in numbers {
        result += n;
    }
    result
}
