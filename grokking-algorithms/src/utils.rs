fn create_subsets(items: &Vec<String>) -> Vec<Vec<String>> {
    let n: usize = items.len();
    let max = 2_u32.pow(n as u32);
    let reversed_items: Vec<String> = items.iter().rev().cloned().collect();
    let mut subsets: Vec<Vec<String>> = Vec::new();
    for i in 0..max {
        let mut subset: Vec<String> = Vec::new();
        let mut mask = 1;
        for j in 0..n {
            if i & mask != 0 {
                subset.push(reversed_items[j].clone());
            }
            mask = mask << 1;
        }
        subset.sort();
        subsets.push(subset);
    }
    subsets.sort();
    subsets.sort_by(|l, r| l.len().cmp(&r.len()));
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
}
