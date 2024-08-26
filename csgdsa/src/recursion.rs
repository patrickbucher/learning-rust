use std::collections::HashMap;

pub fn find_anagrams(word: &str) -> Vec<String> {
    let mut anagrams: Vec<String> = Vec::new();
    if word.len() == 1 {
        return vec![word.to_string()];
    }
    let chars: Vec<char> = word.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        let left = &chars[0..i];
        let right = &chars[(i + 1)..chars.len()];
        let rest = [left, right].concat();
        for part in find_anagrams(&String::from_iter(rest)) {
            anagrams.push(format!("{c}{part}"));
        }
    }
    anagrams
}

pub fn staircase(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1, // [1]
        2 => 2, // [1, 1], [2]
        3 => 4, // [1, 1, 1], [1, 2], [2, 1], [3]
        _ => staircase(n - 1) + staircase(n - 2) + staircase(n - 3),
    }
}

pub fn charcount(words: &[&str]) -> usize {
    if words.is_empty() {
        0
    } else {
        words[0].len() + charcount(&words[1..])
    }
}

pub fn filter_even(numbers: &[isize]) -> Vec<isize> {
    if numbers.is_empty() {
        Vec::new()
    } else if numbers[0] % 2 == 0 {
        [vec![numbers[0]], filter_even(&numbers[1..])].concat()
    } else {
        filter_even(&numbers[1..])
    }
}

pub fn triangular_numbers(n: usize) -> Vec<usize> {
    match n {
        0 => Vec::new(),
        1 => vec![1],
        _ => {
            let left = triangular_numbers(n - 1);
            let next = left[left.len() - 1] + n;
            [left, vec![next]].concat()
        }
    }
}

pub fn find_first_index_of(word: &str, letter: char) -> Option<usize> {
    let chars: Vec<char> = word.chars().collect();
    first_index_of(&chars, letter, 0)
}

fn first_index_of(haystack: &[char], needle: char, index: usize) -> Option<usize> {
    if haystack.is_empty() {
        None
    } else if haystack[0] == needle {
        Some(index)
    } else {
        first_index_of(&haystack[1..], needle, index + 1)
    }
}

pub fn find_unique_paths(rows: usize, cols: usize) -> usize {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::from([((0, 0), 0)]);
    unique_paths_memoized(rows, cols, 0, 0, &mut cache)
}

fn unique_paths_memoized(
    rows: usize,
    cols: usize,
    x: usize,
    y: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if x == rows - 1 && y == rows - 1 {
        1
    } else if x >= rows || y >= cols {
        0
    } else {
        match cache.get(&(rows - x, cols - y)) {
            Some(result) => *result,
            None => {
                let result = unique_paths_memoized(rows, cols, x + 1, y, cache)
                    + unique_paths_memoized(rows, cols, x, y + 1, cache);
                cache.insert((rows - x, cols - y), result);
                result
            }
        }
    }
}

pub fn fibonacci(n: usize) -> usize {
    let mut cache = HashMap::from([(0, 1), (1, 1)]);
    fibonacci_memoized(n, &mut cache)
}

fn fibonacci_memoized(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    match cache.get(&n) {
        Some(result) => *result,
        None => {
            let x = fibonacci_memoized(n - 2, cache);
            let y = fibonacci_memoized(n - 1, cache);
            let z = x + y;
            cache.insert(n, z);
            z
        }
    }
}

pub fn add_until(n: usize, numbers: &[usize]) -> usize {
    if numbers.is_empty() {
        0
    } else {
        let rest = add_until(n, &numbers[1..]);
        if numbers[0] + rest > n {
            rest
        } else {
            numbers[0] + rest
        }
    }
}

pub fn golomb(n: usize) -> usize {
    golomb_memoized(n, &mut HashMap::from([(1, 1)]))
}

fn golomb_memoized(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    match cache.get(&n) {
        Some(result) => *result,
        None => {
            let param = golomb_memoized(n - 1, cache);
            let result = 1 + golomb_memoized(n - param, cache);
            cache.insert(n, result);
            result
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_staircase() {
        assert_eq!(staircase(0), 0);

        // [1]
        assert_eq!(staircase(1), 1);

        // [1, 1], [2]
        assert_eq!(staircase(2), 2);

        // [1, 1, 1], [2, 1], [1, 2], [3]
        assert_eq!(staircase(3), 4);

        /*
         * [1, 1, 1, 1]
         * [2, 1, 1], [1, 2, 1], [1, 1, 2]
         * [2, 2]
         * [3, 1], [1, 3]
         */
        assert_eq!(staircase(4), 7);

        /*
         * [1, 1, 1, 1, 1]
         * [2, 1, 1, 1], [1, 2, 1, 1], [1, 1, 2, 1], [1, 1, 1, 2]
         * [2, 2, 1], [2, 1, 2], [1, 2, 2]
         * [3, 1, 1], [1, 3, 1], [1, 1, 3]
         * [3, 2], [2, 3]
         */
        assert_eq!(staircase(5), 13);
    }

    #[test]
    fn test_anagrams_empty_string() {
        assert_eq!(find_anagrams(""), Vec::<String>::new());
    }

    #[test]
    fn test_anagrams_singleton_string() {
        assert_eq!(find_anagrams("a"), vec!["a"]);
    }

    #[test]
    fn test_anagrams_two_char_string() {
        assert_eq!(find_anagrams("ab"), vec!["ab", "ba"]);
    }

    #[test]
    fn test_anagrams_three_char_string() {
        assert_eq!(
            find_anagrams("abc"),
            vec!["abc", "acb", "bac", "bca", "cab", "cba"]
        );
    }

    #[test]
    fn test_anagrams_multi_char_string_heuristically() {
        assert_eq!(find_anagrams("abcd").len(), 24); // 4! = 24
        assert_eq!(find_anagrams("abcde").len(), 120); // 5! = 120
        assert_eq!(find_anagrams("abcdef").len(), 720); // 6! = 720
    }

    #[test]
    fn test_charcount() {
        let tests: HashMap<Vec<&str>, usize> = HashMap::from([
            (Vec::new(), 0),
            (vec![""], 0),
            (vec!["", "", ""], 0),
            (vec!["a"], 1),
            (vec!["a", "b"], 2),
            (vec!["", "a", "ab", "abc"], 6),
        ]);
        for (input, expected) in tests {
            let actual = charcount(&input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_filter_even() {
        let tests: HashMap<Vec<isize>, Vec<isize>> = HashMap::from([
            (Vec::new(), Vec::new()),
            (vec![1], Vec::new()),
            (vec![1, 2], vec![2]),
            (vec![1, 2, 3], vec![2]),
            (vec![1, 2, 3, 4], vec![2, 4]),
        ]);
        for (input, expected) in tests {
            let actual = filter_even(&input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_triangular_numbers() {
        let tests: HashMap<usize, Vec<usize>> = HashMap::from([
            (0, Vec::new()),
            (1, vec![1]),
            (2, vec![1, 3]),
            (3, vec![1, 3, 6]),
            (4, vec![1, 3, 6, 10]),
            (5, vec![1, 3, 6, 10, 15]),
            (6, vec![1, 3, 6, 10, 15, 21]),
            (7, vec![1, 3, 6, 10, 15, 21, 28]),
            (8, vec![1, 3, 6, 10, 15, 21, 28, 36]),
            (9, vec![1, 3, 6, 10, 15, 21, 28, 36, 45]),
        ]);
        for (input, expected) in tests {
            let actual = triangular_numbers(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_find_first_index_of() {
        let tests: HashMap<(&str, char), Option<usize>> = HashMap::from([
            (("", 'x'), None),
            (("x", 'x'), Some(0)),
            (("xyz", 'x'), Some(0)),
            (("misterx", 'x'), Some(6)),
            (("abcdefghijklmnopqrstuvwxyz", 'x'), Some(23)),
            (("abcdefghijklmnopqrstuvwXyz", 'x'), None),
        ]);
        for ((word, letter), expected) in tests {
            let actual = find_first_index_of(word, letter);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_find_unique_paths() {
        let tests: HashMap<(usize, usize), usize> = HashMap::from([
            ((1, 2), 1),
            ((2, 1), 1),
            ((2, 2), 2),
            ((3, 2), 3),
            ((3, 3), 6),
        ]);
        for ((rows, cols), expected) in tests {
            let actual = find_unique_paths(rows, cols);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_fibonacci() {
        let tests: HashMap<usize, usize> = HashMap::from([
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 5),
            (5, 8),
            (6, 13),
            (7, 21),
            (8, 34),
            (9, 55),
        ]);
        for (input, expected) in tests {
            let actual = fibonacci(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_add_until() {
        let tests: HashMap<Vec<usize>, usize> = HashMap::from([
            (Vec::new(), 0),
            (vec![10], 10),
            (vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 45),
            (vec![0, 1, 2, 3, 4, 50, 5, 6, 7, 8, 9], 95),
            (vec![0, 1, 2, 3, 4, 99, 5, 6, 7, 8, 9], 45),
            (vec![10, 20, 30], 60),
            (vec![10, 20, 30, 40], 100),
            (vec![15, 25, 45, 35], 95),
            (vec![10, 20, 30, 40, 50], 100),
        ]);
        for (input, expected) in tests {
            let actual = add_until(100, &input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_golomb() {
        let tests: HashMap<usize, usize> = HashMap::from([
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 4),
            (8, 4),
            (9, 4),
            (10, 4),
            (11, 5),
            (12, 5),
            (13, 5),
            (14, 5),
            (15, 5),
        ]);
        for (input, expected) in tests {
            let actual = golomb(input);
            assert_eq!(actual, expected);
        }
    }
}
