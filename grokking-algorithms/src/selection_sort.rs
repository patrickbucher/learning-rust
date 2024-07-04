/// Sorts the items in O(n²) by subsequently selecting the smallest item.
///
/// ```
/// use grokking_algorithms::selection_sort::sort;
/// assert_eq!(sort(&vec![3, 4, 1, 0, 2]), vec![0, 1, 2, 3, 4]);
/// ```
pub fn sort<T: Ord + Clone>(items: &Vec<T>) -> Vec<T> {
    let mut sorted: Vec<T> = Vec::new();
    let mut backup: Vec<T> = items.to_owned();
    while !backup.is_empty() {
        if let Some(i) = find_min_index(&backup) {
            sorted.push(backup[i].clone());
            backup.remove(i);
        } else {
            return sorted;
        }
    }
    sorted
}

fn find_min_index<T: Ord>(items: &Vec<T>) -> Option<usize> {
    if items.is_empty() {
        None
    } else {
        let mut min = &items[0];
        let mut min_index = 0;
        for i in 1..items.len() {
            let item = &items[i];
            if item < min {
                min = item;
                min_index = i;
            }
        }
        Some(min_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_vec() {
        let empty: Vec<u8> = Vec::new();
        let expected: Vec<u8> = Vec::new();
        let actual: Vec<u8> = sort(&empty);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sort_singleton_vec() {
        let numbers: Vec<u8> = vec![42];
        let expected: Vec<u8> = vec![42];
        let actual: Vec<u8> = sort(&numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sort_numbers() {
        let numbers: Vec<u8> = vec![7, 3, 4, 1, 9, 0, 2, 6, 5, 8];
        let expected: Vec<u8> = (0..=9).collect();
        let actual: Vec<u8> = sort(&numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_min_index_in_empty_vec() {
        let empty: Vec<u8> = Vec::new();
        let expected = None;
        let actual = find_min_index(&empty);
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_min_index_in_singleton_vec() {
        let numbers: Vec<u8> = vec![42];
        let expected = Some(0);
        let actual = find_min_index(&numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_min_index_in_vec() {
        let numbers: Vec<u8> = (0..=9).rev().collect();
        let expected = Some(numbers.len() - 1);
        let actual = find_min_index(&numbers);
        assert_eq!(actual, expected);
    }
}
