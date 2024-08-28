pub fn quick_sort<T: Clone + Ord>(values: &mut [T]) {
    let n = values.len();
    if n <= 1 {
        return;
    }
    let top = n - 1;
    let pivot = values[top].clone();
    let mut i = 0;
    let mut j = top - 1;
    loop {
        while values[i] < pivot && i < top {
            i += 1;
        }
        while values[j] > pivot && j > 0 {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            let tmp = values[i].clone();
            values[i] = values[j].clone();
            values[j] = tmp;
        }
    }
    let tmp = values[i].clone();
    values[i] = pivot;
    values[top] = tmp;
    quick_sort(&mut values[0..i]);
    quick_sort(&mut values[i..n]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_empty() {
        let mut values: Vec<usize> = Vec::new();
        quick_sort(&mut values);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(values, expected);
    }

    #[test]
    fn quick_sort_sorted() {
        let mut values: Vec<usize> = vec![1, 2, 3];
        quick_sort(&mut values);
        let expected: Vec<usize> = vec![1, 2, 3];
        assert_eq!(values, expected);
    }

    #[test]
    fn quick_sort_unsorted_small() {
        let mut values: Vec<usize> = vec![5, 7, 6, 4, 8, 2, 3, 1, 9, 0];
        quick_sort(&mut values);
        let expected: Vec<usize> = (0..10).collect();
        assert_eq!(values, expected);
    }

    #[test]
    fn quick_sort_unsorted_big() {
        let mut values: Vec<isize> = (-100..100).rev().collect();
        quick_sort(&mut values);
        let expected: Vec<isize> = (-100..100).collect();
        assert_eq!(values, expected);
    }
}
