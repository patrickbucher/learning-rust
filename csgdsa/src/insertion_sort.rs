pub fn insertion_sort<T: Clone + Ord>(values: &mut [T]) {
    for i in 1..values.len() {
        let current = values[i].clone();
        let mut new_index = i;
        for j in (0..i).rev() {
            if current < values[j] {
                values[j + 1] = values[j].clone();
                new_index = j;
            } else {
                break;
            }
        }
        values[new_index] = current.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_empty() {
        let mut values: Vec<usize> = Vec::new();
        insertion_sort(&mut values);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(values, expected);
    }

    #[test]
    fn insertion_sort_sorted() {
        let mut values: Vec<usize> = vec![1, 2, 3];
        insertion_sort(&mut values);
        let expected: Vec<usize> = vec![1, 2, 3];
        assert_eq!(values, expected);
    }

    #[test]
    fn insertion_sort_unsorted_small() {
        let mut values: Vec<usize> = vec![7, 5, 6, 8, 4, 9, 3, 2, 0, 1];
        insertion_sort(&mut values);
        let expected: Vec<usize> = (0..10).collect();
        assert_eq!(values, expected);
    }

    #[test]
    fn insertion_sort_unsorted_big() {
        let mut values: Vec<isize> = (-100..100).rev().collect();
        insertion_sort(&mut values);
        let expected: Vec<isize> = (-100..100).collect();
        assert_eq!(values, expected);
    }
}
