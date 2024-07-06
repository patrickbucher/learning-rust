use std::cmp::Ordering;

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

/// Computes the sum of the given numbers recursively.
///
/// ```
/// use grokking_algorithms::recursion::sum;
/// assert_eq!(sum(&vec![]), 0);
/// assert_eq!(sum(&vec![0]), 0);
/// assert_eq!(sum(&vec![1]), 1);
/// assert_eq!(sum(&vec![1, 2, 3]), 6);
/// assert_eq!(sum(&(1..=100).collect()), 5050);
/// ```
pub fn sum(numbers: &Vec<isize>) -> isize {
    if numbers.is_empty() {
        0
    } else {
        let head: isize = numbers[0];
        let tail: Vec<isize> = numbers.into_iter().skip(1).cloned().collect();
        head + sum(&tail)
    }
}

/// Counts the number of elements in the vector recursively.
///
/// ```
/// use grokking_algorithms::recursion::count;
/// assert_eq!(count::<u8>(&vec![]), 0);
/// assert_eq!(count::<u8>(&vec![0]), 1);
/// assert_eq!(count::<u8>(&vec![1]), 1);
/// assert_eq!(count::<u8>(&vec![1, 2, 3]), 3);
/// assert_eq!(count::<u8>(&(0..100).collect()), 100);
/// ```
pub fn count<T: Clone>(numbers: &Vec<T>) -> usize {
    if numbers.is_empty() {
        0
    } else {
        let tail: Vec<T> = numbers.into_iter().skip(1).cloned().collect();
        1 + count(&tail)
    }
}

/// Finds the maximum number in the vector recursively.
///
/// ```
/// use grokking_algorithms::recursion::max;
/// assert_eq!(max(&vec![]), None);
/// assert_eq!(max(&vec![1]), Some(1));
/// assert_eq!(max(&vec![1, 2, 3]), Some(3));
/// assert_eq!(max(&vec![9, 5, 0]), Some(9));
/// assert_eq!(max(&(0..100).collect()), Some(99));
/// ```
pub fn max(numbers: &Vec<isize>) -> Option<isize> {
    if numbers.is_empty() {
        None
    } else {
        find_max(numbers, None)
    }
}

fn find_max(numbers: &Vec<isize>, acc: Option<isize>) -> Option<isize> {
    if numbers.is_empty() {
        acc
    } else {
        let head = numbers[0];
        let tail: Vec<isize> = numbers.into_iter().skip(1).cloned().collect();
        let acc = match acc {
            Some(val) => {
                if head > val {
                    Some(head)
                } else {
                    Some(val)
                }
            }
            None => Some(head),
        };
        find_max(&tail, acc)
    }
}

/// Finds the index of the needle in the sorted haystack.
///
/// ```
/// use grokking_algorithms::recursion::binary_search;
/// assert_eq!(binary_search(&vec![], 42), None);
/// assert_eq!(binary_search(&vec![0], 0), Some(0));
/// assert_eq!(binary_search(&vec![1, 2, 3], 1), Some(0));
/// assert_eq!(binary_search(&vec![1, 2, 3], 2), Some(1));
/// assert_eq!(binary_search(&vec![1, 2, 3], 3), Some(2));
/// assert_eq!(binary_search(&(0..1000).collect(), 523), Some(523));
/// ```
pub fn binary_search(haystack: &Vec<isize>, needle: isize) -> Option<usize> {
    if haystack.is_empty() {
        None
    } else {
        do_binary_search(haystack, needle, 0, haystack.len() - 1)
    }
}

fn do_binary_search(
    haystack: &Vec<isize>,
    needle: isize,
    low: usize,
    high: usize,
) -> Option<usize> {
    if haystack.is_empty() {
        return None;
    }
    let mid = (high + low) / 2;
    match needle.cmp(&haystack[mid]) {
        Ordering::Equal => Some(mid),
        Ordering::Less => do_binary_search(&haystack, needle, low, mid - 1),
        Ordering::Greater => do_binary_search(&haystack, needle, mid + 1, high),
    }
}
