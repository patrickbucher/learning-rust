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
    unique_paths(rows, cols, 0, 0)
}

fn unique_paths(rows: usize, cols: usize, x: usize, y: usize) -> usize {
    if x == rows - 1 && y == rows - 1 {
        1
    } else if x >= rows || y >= cols {
        0
    } else {
        unique_paths(rows, cols, x + 1, y) + unique_paths(rows, cols, x, y + 1)
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
}
