pub fn split_words(text: &str) -> Vec<&str> {
    let mut words = vec![];
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
    fn test_split() {
        let text = "This  is some    test .";
        let words = split_words(text);
        assert_eq!(vec!["This", "is", "some", "test", "."], words);
    }
}
