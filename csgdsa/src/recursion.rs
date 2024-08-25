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

#[cfg(test)]
pub mod tests {
    use super::*;

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
}
