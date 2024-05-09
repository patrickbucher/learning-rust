enum PasswordStrengh {
    VeryWeak,
    Weak,
    Strong,
    VeryStrong,
    Unknown,
}

impl PasswordStrengh {
    fn of(pw: &str) -> Self {
        if is_very_strong_password(pw) {
            PasswordStrengh::VeryStrong
        } else if is_strong_password(pw) {
            PasswordStrengh::Strong
        } else if is_weak_password(pw) {
            PasswordStrengh::Weak
        } else if is_very_weak_password(pw) {
            PasswordStrengh::VeryWeak
        } else {
            PasswordStrengh::Unknown
        }
    }
    fn designation(self) -> String {
        match self {
            PasswordStrengh::VeryWeak => "very weak".into(),
            PasswordStrengh::Weak => "weak".into(),
            PasswordStrengh::Strong => "strong".into(),
            PasswordStrengh::VeryStrong => "very strong".into(),
            PasswordStrengh::Unknown => "questionable".into(),
        }
    }
}

fn main() {
    let passwords = vec!["12345", "abcdef", "abc123xyz", "1337h@xor!"];
    for pw in passwords {
        let strength = PasswordStrengh::of(pw);
        println!(
            "The password '{pw}' is a {} password.",
            strength.designation()
        );
    }
}

fn is_very_weak_password(pw: &str) -> bool {
    pw.chars().count() < 8 && pw.chars().all(|c| c.is_digit(10))
}

fn is_weak_password(pw: &str) -> bool {
    pw.chars().count() < 8 && pw.chars().all(|c| c.is_ascii_alphabetic())
}

fn is_strong_password(pw: &str) -> bool {
    pw.chars().count() >= 8 && has_digits(pw) && has_letters(pw)
}

fn is_very_strong_password(pw: &str) -> bool {
    pw.chars().count() >= 8 && has_letters(pw) && has_digits(pw) && has_special_characters(pw)
}

fn has_digits(s: &str) -> bool {
    s.chars().any(|c| c.is_digit(10))
}

fn has_letters(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_alphabetic())
}

fn has_special_characters(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_punctuation())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_very_weak_password() {
        assert!(is_very_weak_password("1234567"));
    }

    #[test]
    fn test_not_is_very_weak_password() {
        assert!(!is_very_weak_password("123456789"));
        assert!(!is_very_weak_password("123abc"));
    }

    #[test]
    fn test_is_weak_password() {
        assert!(is_weak_password("abcdef"));
    }

    #[test]
    fn test_not_is_weak_password() {
        assert!(!is_weak_password("abcdefghij"));
        assert!(!is_weak_password("abcdef1"));
    }

    #[test]
    fn test_is_strong_password() {
        assert!(is_strong_password("abcd1234"));
    }

    #[test]
    fn test_not_is_strong_password() {
        assert!(!is_strong_password("abcd123"));
        assert!(!is_strong_password("abcdefghij"));
        assert!(!is_strong_password("123456789"));
    }

    #[test]
    fn test_is_very_strong_password() {
        assert!(is_very_strong_password("foo#bar3"));
    }

    #[test]
    fn test_not_is_very_strong_password() {
        assert!(!is_very_strong_password("foobar378"));
    }

    #[test]
    fn test_has_digits() {
        assert!(has_digits("1337"));
        assert!(has_digits("this is s7range"));
        assert!(has_digits("1 digit 2"));
    }

    #[test]
    fn test_not_has_digits() {
        assert!(!has_digits(""));
        assert!(!has_digits("no digits"));
        assert!(!has_digits("no#thing."));
    }

    #[test]
    fn test_has_letters() {
        assert!(has_letters("abc"));
    }

    #[test]
    fn test_not_has_letters() {
        assert!(!has_letters("1337"));
    }

    #[test]
    fn test_has_special_characters() {
        assert!(has_special_characters("foo.bar"));
    }

    #[test]
    fn test_not_has_special_characters() {
        assert!(!has_special_characters("foobar"));
    }
}
