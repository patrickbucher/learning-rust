pub fn find_greatest<T: Ord + Clone>(items: &[T]) -> Option<T> {
    let mut greatest = None;
    for item in items {
        greatest = match greatest {
            Some(value) => {
                if item > value {
                    Some(item)
                } else {
                    Some(value)
                }
            }
            None => Some(item),
        }
    }
    greatest.cloned()
}

pub fn intersect<T: Eq + Clone>(xs: &[T], ys: &[T]) -> Vec<T> {
    let mut intersection: Vec<T> = Vec::new();
    for x in xs {
        for y in ys {
            if x == y {
                intersection.push(x.clone());
                break;
            }
        }
    }
    intersection
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_no_greatest_number_in_empty_vec() {
        let numbers: Vec<usize> = Vec::new();
        let expected = None;
        let actual = find_greatest(&numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_greatest_number_in_singleton_vec() {
        let numbers: Vec<usize> = vec![7];
        let expected = Some(7);
        let actual = find_greatest(&numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_greatest_number_in_vec() {
        let numbers: Vec<usize> = vec![2, 8, 5, 9, 1, 3];
        let expected = Some(9);
        let actual = find_greatest(&numbers);
        assert_eq!(actual, expected);
    }

    #[test]
    fn intersection_of_empty_vecs() {
        let xs: Vec<usize> = Vec::new();
        let ys: Vec<usize> = Vec::new();
        let actual: Vec<usize> = intersect(&xs, &ys);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn intersection_of_non_empty_and_empty_vec() {
        let non_empty: Vec<usize> = vec![1, 2, 3];
        let empty: Vec<usize> = Vec::new();
        let actual: Vec<usize> = intersect(&non_empty, &empty);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn intersection_of_disjunct_vecs() {
        let odds: Vec<usize> = vec![1, 3, 5, 7, 9];
        let evens: Vec<usize> = vec![0, 2, 4, 6, 8];
        let actual: Vec<usize> = intersect(&odds, &evens);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn intersection_of_conjoint_vecs() {
        let twos: Vec<usize> = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        let threes: Vec<usize> = vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30];
        let actual: Vec<usize> = intersect(&twos, &threes);
        let expected: Vec<usize> = vec![6, 12, 18];
        assert_eq!(actual, expected);
    }
}
