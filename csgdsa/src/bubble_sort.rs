pub fn bubble_sort<T: Clone + Ord>(values: &mut Vec<T>) {
    for i in 0..values.len() {
        let mut sorted = true;
        for j in 1..(values.len() - i) {
            if values[j - 1] > values[j] {
                sorted = false;
                let tmp = values[j].clone();
                values[j] = values[j - 1].clone();
                values[j - 1] = tmp;
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_empty() {
        let mut values: Vec<usize> = Vec::new();
        bubble_sort(&mut values);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(values, expected);
    }

    #[test]
    fn bubble_sort_sorted() {
        let mut values: Vec<usize> = vec![1, 2, 3];
        bubble_sort(&mut values);
        let expected: Vec<usize> = vec![1, 2, 3];
        assert_eq!(values, expected);
    }

    #[test]
    fn bubble_sort_unsorted_small() {
        let mut values: Vec<usize> = vec![5, 6, 4, 7, 3, 8, 2, 9, 1, 0];
        bubble_sort(&mut values);
        let expected: Vec<usize> = (0..10).collect();
        assert_eq!(values, expected);
    }

    #[test]
    fn bubble_sort_unsorted_big() {
        let mut values: Vec<isize> = (-100..100).rev().collect();
        bubble_sort(&mut values);
        let expected: Vec<isize> = (-100..100).collect();
        assert_eq!(values, expected);
    }
}
