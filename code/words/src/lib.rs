pub fn reverse_words(text: &str) -> String {
    let mut reversed_text = String::new();
    let mut reversed: Vec<&str> = Vec::new();
    let words = split_words(text);
    for i in 0..words.len() {
        let j = words.len() - 1 - i;
        reversed.push(words[j])
    }
    for word in reversed {
        if reversed_text != "" {
            reversed_text.push(' ');
        }
        reversed_text.push_str(&word);
    }
    reversed_text
}

fn split_words(text: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let mut i = 0;
    let mut in_word = true;
    let bytes = text.as_bytes();
    for (j, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            if in_word {
                words.push(&text[i..j]);
                in_word = false;
            }
        } else if !in_word {
            in_word = true;
            i = j // start of next word
        }
    }
    words.push(&text[i..]);
    words
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse() {
        let text = "This is some test";
        let reversed = reverse_words(text);
        assert_eq!("test some is This", reversed);
    }

    #[test]
    fn test_split() {
        let text = "This is some test";
        let words = split_words(text);
        assert_eq!(vec!["This", "is", "some", "test"], words);
    }
}
