use std::cmp::Ordering;

// FIXME: re-implement in sc/quick_sort.rs using the patition function
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
