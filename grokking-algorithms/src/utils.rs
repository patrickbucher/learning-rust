pub fn create_subsets<T: Ord + Clone>(items: &[T]) -> Vec<Vec<T>> {
    let n: usize = items.len();
    let max = 2_u32.pow(n as u32);
    let reversed_items: Vec<T> = items.iter().rev().cloned().collect();
    let mut subsets: Vec<Vec<T>> = Vec::new();
    for i in 0..max {
        let mut subset: Vec<T> = Vec::new();
        let mut mask = 1;
        for items in reversed_items.clone().into_iter().take(n) {
            if i & mask != 0 {
                subset.push(items);
            }
            mask <<= 1;
        }
        subset.sort();
        subsets.push(subset);
    }
    subsets.sort();
    subsets.sort_by_key(|l| l.len());
    subsets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsets_of_empty_set() {
        let items: Vec<String> = Vec::new();
        let expected: Vec<Vec<String>> = vec![vec![]];
        let actual: Vec<Vec<String>> = create_subsets(&items);
        assert_eq!(actual, expected);
    }

    #[test]
    fn subsets_of_nonempty_set() {
        let items: Vec<String> = vec!["a", "b", "c"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let expected: Vec<Vec<String>> = vec![
            vec![],
            vec!["a"].into_iter().map(|s| s.to_string()).collect(),
            vec!["b"].into_iter().map(|s| s.to_string()).collect(),
            vec!["c"].into_iter().map(|s| s.to_string()).collect(),
            vec!["a", "b"].into_iter().map(|s| s.to_string()).collect(),
            vec!["a", "c"].into_iter().map(|s| s.to_string()).collect(),
            vec!["b", "c"].into_iter().map(|s| s.to_string()).collect(),
            vec!["a", "b", "c"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        ];
        let actual: Vec<Vec<String>> = create_subsets(&items);
        assert_eq!(actual, expected);
    }

    #[test]
    fn subsets_of_numeric_set() {
        let items: Vec<usize> = vec![0, 1, 2, 3];
        let expected: Vec<Vec<usize>> = vec![
            vec![],
            vec![0],
            vec![1],
            vec![2],
            vec![3],
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![0, 2, 3],
            vec![1, 2, 3],
            vec![0, 1, 2, 3],
        ];
        let actual: Vec<Vec<usize>> = create_subsets(&items);
        assert_eq!(actual, expected);
    }
}
