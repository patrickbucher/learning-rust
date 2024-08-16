pub fn selection_sort<T: Clone + Ord>(values: &mut [T]) {
    for i in 0..values.len() {
        let mut smallest_index = i;
        for j in (i + 1)..values.len() {
            if values[j] < values[smallest_index] {
                smallest_index = j;
            }
        }
        if i != smallest_index {
            let tmp = values[i].clone();
            values[i] = values[smallest_index].clone();
            values[smallest_index] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sort_empty() {
        let mut values: Vec<usize> = Vec::new();
        selection_sort(&mut values);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(values, expected);
    }

    #[test]
    fn selection_sort_sorted() {
        let mut values: Vec<usize> = vec![1, 2, 3];
        selection_sort(&mut values);
        let expected: Vec<usize> = vec![1, 2, 3];
        assert_eq!(values, expected);
    }

    #[test]
    fn selection_sort_unsorted_small() {
        let mut values: Vec<usize> = vec![5, 6, 4, 7, 3, 8, 2, 9, 1, 0];
        selection_sort(&mut values);
        let expected: Vec<usize> = (0..10).collect();
        assert_eq!(values, expected);
    }

    #[test]
    fn selection_sort_unsorted_big() {
        let mut values: Vec<isize> = (-100..100).rev().collect();
        selection_sort(&mut values);
        let expected: Vec<isize> = (-100..100).collect();
        assert_eq!(values, expected);
    }
}
