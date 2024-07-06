pub fn sort<T: Ord + Clone>(items: &Vec<T>) -> Vec<T> {
    let n = items.len();
    if n < 2 {
        items.to_owned()
    } else {
        let mid = n / 2;
        let pivot = items[mid].clone();

        let smaller: Vec<T> = items.iter().filter(|i| *i < &pivot).cloned().collect();
        let bigger: Vec<T> = items.iter().filter(|i| *i > &pivot).cloned().collect();
        let left = sort(&smaller);
        let right = sort(&bigger);

        let mut result: Vec<T> = Vec::new();
        result.extend(left);
        result.push(pivot);
        result.extend(right);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_vec() {
        let items: Vec<u8> = Vec::new();
        let expected: Vec<u8> = Vec::new();
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sort_singleton_vec() {
        let items: Vec<u8> = vec![42];
        let expected: Vec<u8> = vec![42];
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sort_small_vec() {
        let items: Vec<u8> = vec![5, 7, 3, 2, 9, 0, 1, 4, 6, 8];
        let expected: Vec<u8> = (0..=9).collect();
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sort_large_vec() {
        let items: Vec<usize> = (0..1000).rev().collect();
        let expected: Vec<usize> = (0..1000).collect();
        let actual = sort(&items);
        assert_eq!(expected, actual);
    }
}
