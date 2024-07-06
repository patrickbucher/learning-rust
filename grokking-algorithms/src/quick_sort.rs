/// Sorts the items in O(n * log n) on average using the quick sort algorithm
///
/// ```
/// use grokking_algorithms::quick_sort::sort;
/// assert_eq!(sort(&Vec::<u8>::new()), Vec::<u8>::new());
/// assert_eq!(sort(&vec![3, 1, 2]), vec![1, 2, 3]);
/// assert_eq!(sort(&vec![5, 6, 4, 7, 3, 8, 2, 9, 1, 0]), (0..=9).collect::<Vec<u8>>());
/// ```
pub fn sort<T: Ord + Clone>(items: &Vec<T>) -> Vec<T> {
    let n = items.len();
    if n < 2 {
        items.to_owned()
    } else {
        let mid = n / 2;
        let pivot = items[mid].clone();

        let smaller: Vec<T> = items.iter().filter(|i| *i < &pivot).cloned().collect();
        let bigger: Vec<T> = items.iter().filter(|i| *i > &pivot).cloned().collect();
        let left = sort(&smaller);
        let right = sort(&bigger);

        let mut result: Vec<T> = Vec::new();
        result.extend(left);
        result.push(pivot);
        result.extend(right);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_vec() {
        let items: Vec<u8> = Vec::new();
        let expected: Vec<u8> = Vec::new();
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sort_singleton_vec() {
        let items: Vec<u8> = vec![42];
        let expected: Vec<u8> = vec![42];
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sort_small_vec() {
        let items: Vec<u8> = vec![5, 7, 3, 2, 9, 0, 1, 4, 6, 8];
        let expected: Vec<u8> = (0..=9).collect();
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sort_large_vec() {
        let items: Vec<usize> = (0..1000).rev().collect();
        let expected: Vec<usize> = (0..1000).collect();
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }
}
