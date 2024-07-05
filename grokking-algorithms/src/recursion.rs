/// Computes the factorial of the given number.
///
/// ```
/// use grokking_algorithms::recursion::factorial;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(2), 2);
/// assert_eq!(factorial(3), 6);
/// assert_eq!(factorial(4), 24);
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
