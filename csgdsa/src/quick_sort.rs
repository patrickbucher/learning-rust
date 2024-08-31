use std::cmp::Ordering;

pub fn quick_sort<T: Clone + Ord>(values: &mut [T]) {
    let n = values.len();
    if n <= 1 {
        return;
    }
    let i = partition(values, 0, n);
    quick_sort(&mut values[0..i]);
    quick_sort(&mut values[i..n]);
}

pub fn quick_select<T: Clone + Ord>(values: &mut [T], selection: isize) -> Option<T> {
    let n = values.len();
    let nth: usize = if selection < 0 {
        ((n as isize) + selection) as usize
    } else {
        selection as usize
    };
    if nth > n {
        return None;
    }
    quick_select_bound(values, nth, 0, n)
}

fn quick_select_bound<T: Clone + Ord>(
    values: &mut [T],
    nth: usize,
    lower: usize,
    upper: usize,
) -> Option<T> {
    if upper - lower == 1 && nth < upper {
        return Some(values[nth].clone());
    } else if (upper as isize) - (lower as isize) < 2 {
        return None;
    }
    let i = partition(values, lower, upper);
    match i.cmp(&nth) {
        Ordering::Equal => Some(values[i].clone()),
        Ordering::Less => quick_select_bound(values, nth, i, upper),
        Ordering::Greater => quick_select_bound(values, nth, lower, i),
    }
}

fn partition<T: Clone + Ord>(values: &mut [T], lower: usize, upper: usize) -> usize {
    let pivot_index = upper - 1;
    let pivot_value = values[pivot_index].clone();
    let mut i = lower;
    let mut j = pivot_index - 1;
    loop {
        while values[i] <= pivot_value && i < pivot_index {
            i += 1;
        }
        while values[j] > pivot_value && j > 0 {
            j -= 1;
        }
        if i >= j {
            break;
        }
        if values[i] > values[j] {
            let tmp = values[i].clone();
            values[i] = values[j].clone();
            values[j] = tmp;
        }
    }
    let tmp = values[i].clone();
    values[i] = pivot_value;
    values[pivot_index] = tmp;
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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

    #[test]
    fn quick_sort_various() {
        let tests: Vec<(Vec<usize>, Vec<usize>)> = vec![
            (Vec::new(), Vec::new()),
            (vec![0], vec![0]),
            (vec![1, 2, 3], vec![1, 2, 3]),
            (vec![2, 3, 1], vec![1, 2, 3]),
            (vec![3, 1, 2], vec![1, 2, 3]),
            (vec![3, 2, 1], vec![1, 2, 3]),
            (vec![1, 2, 3, 1], vec![1, 1, 2, 3]),
        ];
        for (mut test, expected) in tests {
            quick_sort(&mut test);
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_quick_select() {
        let tests: HashMap<(Vec<usize>, isize), Option<usize>> = HashMap::from([
            ((Vec::new(), 0), None),
            ((Vec::new(), -1), None),
            ((vec![50], 0), Some(50)),
            ((vec![50], -1), Some(50)),
            ((vec![10, 20, 30, 40, 50], 0), Some(10)),
            ((vec![10, 20, 30, 40, 50], 1), Some(20)),
            ((vec![10, 20, 30, 40, 50], 2), Some(30)),
            ((vec![10, 20, 30, 40, 50], 3), Some(40)),
            ((vec![10, 20, 30, 40, 50], 4), Some(50)),
            ((vec![10, 20, 30, 40, 50], 5), None),
            ((vec![10, 20, 30, 40, 50], -1), Some(50)),
            ((vec![10, 20, 30, 40, 50], -2), Some(40)),
            ((vec![10, 20, 30, 40, 50], -3), Some(30)),
            ((vec![10, 20, 30, 40, 50], -4), Some(20)),
            ((vec![10, 20, 30, 40, 50], -5), Some(10)),
            ((vec![10, 20, 30, 40, 50], -6), None),
        ]);
        for ((values, selection), expected) in tests {
            let mut values = values.clone();
            let actual = quick_select(&mut values, selection);
            assert_eq!(actual, expected);
        }
    }
}
