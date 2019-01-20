fn main() {
    let words = vec!["first", "apple", "whatever", "boring"];
    for w in words {
        println!("{} -> {}", w, pig_latin(w));
    }
}

// first -> irst-f -> irst-fay
// apple -> apple -> apple-hay
fn pig_latin(s: &str) -> String {
    if let Some(first) = s.to_lowercase().chars().next() {
        if is_vowel(first) {
            return format!("{}-hay", s);
        } else {
            let tail: String = s.chars().skip(1).collect();
            return format!("{}-{}ay", tail, first);
        }
    }
    "".to_string()
}

fn is_vowel(c: char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    for v in vowels {
        if c == v {
            return true;
        }
    }
    false
}
