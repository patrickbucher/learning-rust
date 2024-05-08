use akshually::io::prompt_line;

fn main() {
    println!("Enter two strings and I'll tell you if they are anagrams:");
    let first: String = prompt_line("Enter the first string: ").unwrap();
    let second: String = prompt_line("Enter the second string: ").unwrap();
    if are_anagrams(&first, &second) {
        println!("{} and {} are anagrams", first, second);
    } else {
        println!("{} and {} are NOT anagrams", first, second);
    }
}

fn are_anagrams(first: &str, second: &str) -> bool {
    let mut remainder: Vec<char> = first.chars().collect();
    for c in second.chars() {
        match remainder.iter().position(|&x| x == c) {
            Some(i) => {
                remainder.remove(i);
            }
            None => {
                return false;
            }
        }
    }
    remainder.is_empty()
}

#[cfg(test)]
mod tests {
    use super::are_anagrams;

    #[test]
    fn test_are_anagrams() {
        assert!(are_anagrams("", ""));
        assert!(are_anagrams("abc", "abc"));
        assert!(are_anagrams("abc", "cab"));
        assert!(are_anagrams("foobar", "raboof"));
        assert!(are_anagrams("dayum", "muday"));
    }

    #[test]
    fn test_not_are_anagrams() {
        assert!(!are_anagrams("abc", "abcd")); // different length
        assert!(!are_anagrams("foobar", "fobaar")); // different characters
        assert!(!are_anagrams("Foobar", "raboof")); // different case
    }
}
