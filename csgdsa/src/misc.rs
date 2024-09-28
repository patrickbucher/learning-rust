use crate::quick_sort::quick_sort;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

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

pub fn is_palindrome(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    if chars.is_empty() {
        return true;
    }
    let mut i: usize = 0;
    let mut j: usize = chars.len() - 1;
    while i <= j && j > 0 {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

pub fn find_first_duplicate<'a>(strings: &[&'a str]) -> Option<&'a str> {
    let mut seen: HashSet<&str> = HashSet::new();
    for s in strings {
        if seen.contains(s) {
            return Some(s);
        }
        seen.insert(s);
    }
    None
}

pub fn find_missing_alphabet_letter(text: &str) -> Option<char> {
    let mut seen: HashMap<char, bool> = iter::zip('a'..='z', iter::repeat(false)).collect();
    for letter in text.chars() {
        seen.entry(letter).and_modify(|v| *v = true);
    }
    let unseen: HashMap<char, bool> = seen.into_iter().filter(|(_, v)| !(*v)).collect();
    let mut unseen: Vec<char> = unseen.keys().cloned().collect();
    unseen.sort();
    unseen.first().copied()
}

pub fn find_first_unique_letter(text: &str) -> Option<char> {
    let mut seen: HashMap<char, bool> = HashMap::new();
    for letter in text.chars() {
        seen.entry(letter)
            .and_modify(|v| *v = true)
            .or_insert(false);
    }
    for letter in text.chars() {
        if let Some(duplicate) = seen.get(&letter) {
            if *duplicate {
                continue;
            } else {
                return Some(letter);
            }
        }
    }
    None
}

pub fn has_duplicate<T: Ord + Clone>(values: &mut [T]) -> bool {
    if values.is_empty() {
        return false;
    }
    quick_sort(values);
    for i in 0..(values.len() - 1) {
        if values[i] == values[i + 1] {
            return true;
        }
    }
    false
}

pub fn greatest_product_of_three(values: &[usize]) -> Option<usize> {
    let mut max = 0;
    if values.len() < 3 {
        return None;
    }
    for (a, x) in values.iter().enumerate() {
        for (b, y) in values.iter().enumerate() {
            for (c, z) in values.iter().enumerate() {
                if a != b && b != c && a != c {
                    let product = x * y * z;
                    if product > max {
                        max = product;
                    }
                }
            }
        }
    }
    Some(max)
}

pub fn greatest_product_of_three_optimized(values: &[usize]) -> Option<usize> {
    if values.len() < 3 {
        return None;
    }
    let mut values: Vec<usize> = values.to_vec();
    quick_sort(&mut values);
    let factors = values.iter().rev().take(3);
    Some(factors.product::<usize>())
}

#[allow(clippy::manual_find)]
pub fn find_missing_number(values: &[usize]) -> Option<usize> {
    for (i, _) in values.iter().enumerate() {
        if !values.contains(&i) {
            return Some(i);
        }
    }
    None
}

pub fn find_missing_number_optimized(values: &[usize]) -> Option<usize> {
    let mut values: Vec<usize> = values.to_vec();
    quick_sort(&mut values);
    (0..values.len()).find(|&i| i != values[i])
}

pub fn find_greatest_on2(values: &[usize]) -> Option<usize> {
    if values.is_empty() {
        return None;
    }
    let mut global_max = 0;
    for x in values {
        let mut local_max = 0;
        for y in values {
            if y > x {
                local_max = *y;
            }
        }
        if local_max > global_max {
            global_max = local_max
        }
    }
    Some(global_max)
}

pub fn find_greatest_on_logn(values: &[usize]) -> Option<usize> {
    if values.is_empty() {
        return None;
    }
    let mut values = values.to_vec();
    quick_sort(&mut values);
    Some(values[values.len() - 1])
}

pub fn find_greatest_on(values: &[usize]) -> Option<usize> {
    if values.is_empty() {
        return None;
    }
    let mut max = 0;
    for x in values {
        if *x > max {
            max = *x;
        }
    }
    Some(max)
}

pub fn reverse<T: Clone>(xs: &mut Vec<T>) {
    let n = xs.len();
    for i in 0..n {
        let j = n - 1 - i;
        if i >= j {
            break;
        }
        let tmp = xs[i].clone();
        xs[i] = xs[j].clone();
        xs[j] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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

    #[test]
    fn check_is_palindrome() {
        let tests: HashMap<&str, bool> = HashMap::from([
            ("", true),
            ("x", true),
            ("oo", true),
            ("oh", false),
            ("wow", true),
            ("and", false),
            ("deed", true),
            ("dear", false),
            ("sugus", true),
            ("joint", false),
            ("maoam", true),
            ("fails", false),
            ("kayak", true),
            ("kojak", false),
            ("hannah", true),
            ("heather", false),
            ("deified", true),
            ("dedicated", false),
            ("racecar", true),
            ("ripcurl", false),
            ("rotator", true),
            ("reindeer", false),
            ("reliefpfeiler", true),
            ("gartenzaun", false),
        ]);
        for (word, expected) in tests {
            let actual = is_palindrome(word);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn find_first_duplicates() {
        let tests: HashMap<Vec<&str>, Option<&str>> = HashMap::from([
            (Vec::new(), None),
            (vec!["foo"], None),
            (vec!["foo", "bar"], None),
            (vec!["a", "b", "c", "a"], Some("a")),
            (vec!["a", "b", "c", "d", "d"], Some("d")),
        ]);
        for (test, expected) in tests {
            let actual = find_first_duplicate(&test);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn find_missing_alphabet_letters() {
        let tests: HashMap<&str, Option<char>> = HashMap::from([
            ("", Some('a')),
            ("abc", Some('d')),
            ("abcdefghijklmnopqrstuvwxy", Some('z')),
            ("abcdefghijklmnopqrstuvwxyz", None),
            ("the quick brown box jumps over a lazy dog", Some('f')),
        ]);
        for (test, expected) in tests {
            let actual = find_missing_alphabet_letter(test);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn find_first_unique_letters() {
        let tests: HashMap<&str, Option<char>> = HashMap::from([
            ("", None),
            ("abc", Some('a')),
            ("abca", Some('b')),
            ("abcd", Some('a')),
            ("minimum", Some('n')),
        ]);
        for (test, expected) in tests {
            let actual = find_first_unique_letter(test);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn has_duplicates() {
        let tests: HashMap<Vec<usize>, bool> = HashMap::from([
            (Vec::new(), false),
            (vec![1], false),
            (vec![1, 1], true),
            (vec![1, 2, 3], false),
            (vec![1, 2, 1], true),
            (vec![1, 2, 3, 3], true),
            (vec![1, 2, 3, 4, 2], true),
            (vec![1, 2, 3, 1, 2, 3, 1, 2, 3], true),
        ]);
        for (mut test, expected) in tests {
            let actual = has_duplicate(&mut test);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn greatest_products_of_three() {
        let tests: HashMap<Vec<usize>, Option<usize>> = HashMap::from([
            (Vec::new(), None),
            (vec![1], None),
            (vec![1, 2], None),
            (vec![1, 2, 3], Some(6)),
            (vec![1, 3, 3], Some(9)),
            (vec![1, 2, 3, 2, 1], Some(12)),
            (vec![5, 6, 4, 7, 4], Some(210)),
        ]);
        for (test, expected) in tests {
            let actual = greatest_product_of_three(&test);
            assert_eq!(actual, expected);
            let actual = greatest_product_of_three_optimized(&test);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn find_missing_numbers() {
        let tests: HashMap<Vec<usize>, Option<usize>> = HashMap::from([
            (vec![0], None),
            (vec![1, 0], None),
            (vec![0, 2, 1], None),
            (vec![3, 0, 2, 1], None),
            (vec![3, 0, 4, 2, 1], None),
            (vec![3, 5, 0, 4, 2, 1], None),
            (vec![3, 5, 0, 4, 2, 1, 6], None),
            (vec![3, 5, 0, 7, 4, 2, 1, 6], None),
            (vec![8, 3, 5, 0, 7, 4, 2, 1, 6], None),
            (vec![8, 3, 5, 0, 7, 4, 9, 2, 1, 6], None),
            (vec![2, 0], Some(1)),
            (vec![1, 3, 0], Some(2)),
            (vec![3, 0, 2, 5], Some(1)),
            (vec![3, 0, 9, 2, 1], Some(4)),
            (vec![3, 5, 0, 4, 2, 9], Some(1)),
            (vec![3, 5, 0, 4, 2, 1, 7], Some(6)),
            (vec![3, 8, 0, 7, 4, 2, 1, 6], Some(5)),
            (vec![9, 3, 5, 0, 7, 4, 2, 1, 6], Some(8)),
            (vec![8, 3, 5, 10, 0, 4, 9, 2, 1, 6], Some(7)),
        ]);
        for (test, expected) in tests {
            let actual = find_missing_number(&test);
            assert_eq!(actual, expected);
            let actual = find_missing_number_optimized(&test);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn find_greatest_numbers() {
        let tests: HashMap<Vec<usize>, Option<usize>> = HashMap::from([
            (Vec::new(), None),
            (vec![0], Some(0)),
            (vec![0, 1], Some(1)),
            (vec![2, 0, 1], Some(2)),
            (vec![2, 3, 0, 1], Some(3)),
            (vec![4, 2, 3, 0, 1], Some(4)),
            (vec![4, 2, 3, 0, 1, 5], Some(5)),
        ]);
        for (test, expected) in tests {
            assert_eq!(find_greatest_on2(&test), expected);
            assert_eq!(find_greatest_on_logn(&test), expected);
            assert_eq!(find_greatest_on(&test), expected);
        }
    }

    #[test]
    fn reverse_vec() {
        let mut numbers: Vec<usize> = (0..10).collect();
        let expected: Vec<usize> = (0..10).rev().collect();
        reverse(&mut numbers);
        assert_eq!(expected, numbers);
    }
}
