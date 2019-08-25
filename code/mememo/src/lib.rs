//! # mememo
//!
//! The crate _mememo_ provides trivial implementations of the operations _mean_, _median_, and
//! _mode_. This crate is not intended for productive use, but only to demonstrate the use of
//! crates and other Rust features.

use std::collections::HashMap;

/// Calculates the mean of the elements in the given vector.
///
/// # Example
///
/// ```
/// let numbers = vec![1, 2, 3, 4];
/// assert_eq!(2.5, mememo::mean(&numbers));
/// ```
pub fn mean(numbers: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in numbers {
        sum += i;
    }
    sum as f64 / numbers.len() as f64
}

pub enum Median {
    MiddleSingle(i32),
    MiddleTwoMean(f64),
}

/// Calculates the median of the elements in the given vector.
///
/// # Example
///
/// For a vector with an _odd_ number of elements, the value of the middle
/// element will be returned:
///
/// ```
/// let numbers = vec![1, 2, 3, 4, 5];
/// match mememo::median(&numbers) {
///     mememo::Median::MiddleSingle(got) => assert_eq!(3, got),
///     _ => panic!("wrong median calculation"),
/// }
/// ```
///
/// For a vector with an _even_ number of elements, the mean of the two middle
/// elements will be calculated:
///
/// ```
/// let numbers = vec![1, 2, 3, 4];
/// match mememo::median(&numbers) {
///     mememo::Median::MiddleTwoMean(got) => assert_eq!(2.5, got),
///     _ => panic!("wrong median calculation"),
/// };
/// ```
pub fn median(numbers: &Vec<i32>) -> Median {
    let i = numbers.len() / 2;
    if numbers.len() % 2 == 0 && numbers.len() >= 2 {
        // even: mean of middle two
        Median::MiddleTwoMean((numbers[i - 1] + numbers[i]) as f64 / 2.0)
    } else {
        Median::MiddleSingle(numbers[i])
    }
}

/// Calculates the mode of the elements in the given vector.
///
/// # Example
///
/// ```rust
/// let numbers = vec![1, 2, 3, 1, 2, 1];
/// assert_eq!(1, mememo::mode(&numbers));
/// ```
pub fn mode(numbers: &Vec<i32>) -> i32 {
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
