// TODO:
// - separate values in left, median, right
// - add the median to the list
// - continue recursively with left
// - continue recursively with right

pub fn middle_out<T: Ord + Clone>(values: &Vec<T>) -> Vec<T> {
    if values.is_empty() {
        Vec::<T>::new()
    } else {
        let mut result = Vec::<T>::new();
        let mut values = values.to_owned();
        while let (Some(median), rest) = split_median(&values) {
            values = rest;
            result.push(median);
        }
        result
    }
}

fn split_median<T: Ord + Clone>(values: &[T]) -> (Option<T>, Vec<T>) {
    let n = values.len();
    if n == 0 {
        (None, Vec::new())
    } else if n == 1 {
        (Some(values[0].clone()), Vec::new())
    } else {
        let mut values = values.to_owned();
        values.sort();
        let m = if n % 2 == 0 { n / 2 - 1 } else { n / 2 };
        let left = &values[0..m];
        let right = &values[m + 1..n];
        (Some(values[m].clone()), [left, right].concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_out() {
        assert_eq!(middle_out(&Vec::<usize>::new()), Vec::new());
        assert_eq!(middle_out(&vec![3]), vec![3]);
        assert_eq!(middle_out(&vec![3, 2]), vec![2, 3]);
        assert_eq!(middle_out(&vec![1, 3, 2]), vec![2, 1, 3]);
        assert_eq!(middle_out(&vec![4, 1, 3, 2]), vec![2, 3, 1, 4]);
        assert_eq!(middle_out(&vec![4, 1, 3, 2, 5]), vec![3, 2, 4, 1, 5]);
        // TODO: this is bogus, too
    }

    #[test]
    fn test_split_median() {
        assert_eq!(split_median(&Vec::<usize>::new()), (None, Vec::new()));
        assert_eq!(split_median(&vec![3]), (Some(3), Vec::new()));
        assert_eq!(split_median(&vec![4, 3]), (Some(3), vec![4]));
        assert_eq!(split_median(&vec![2, 4, 3]), (Some(3), vec![2, 4]));
        assert_eq!(split_median(&vec![2, 4, 3, 1]), (Some(2), vec![1, 3, 4]));
    }
}
