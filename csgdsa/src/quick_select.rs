use std::cmp::Ordering;

pub fn quick_select<T: Clone + Ord>(values: &mut [T], selection: isize) -> Option<T> {
    if values.is_empty() {
        return None;
    }
    let n = values.len();
    let pivot_index = partition(values, 0, n - 1);
    let selection: usize = if selection >= 0 {
        selection as usize
    } else if (n as isize + selection) >= 0 {
        ((n as isize) + selection) as usize
    } else {
        return None;
    };
    if selection >= values.len() {
        return None;
    }
    match pivot_index.cmp(&selection) {
        Ordering::Equal => Some(values[pivot_index].clone()),
        Ordering::Less => {
            let index = partition(values, 0, pivot_index);
            Some(values[index].clone())
        }
        Ordering::Greater => {
            let index = partition(values, pivot_index, n);
            Some(values[index].clone())
        }
    }
}

// TODO: re-use partition for quick sort

fn partition<T: Clone + Ord>(values: &mut [T], lower: usize, upper: usize) -> usize {
    let n = upper - lower;
    if n == 0 {
        return 0;
    }
    let pivot_index = upper - 1;
    let pivot_value = values[pivot_index].clone();
    let mut i = lower;
    let mut j = pivot_index - 1;
    loop {
        while values[i] < pivot_value && i < pivot_index {
            i += 1;
        }
        while values[j] > pivot_value && j > 0 {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            let tmp = values[i].clone();
            values[i] = pivot_value.clone();
            values[pivot_index] = tmp;
        }
    }
    let tmp = values[i].clone();
    values[i] = pivot_value;
    values[pivot_index] = tmp;
    pivot_index
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    #[ignore]
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
