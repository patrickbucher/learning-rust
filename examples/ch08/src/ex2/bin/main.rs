fn main() {
    let text = String::from("everything is a bit odd today");
    let latin = pig_latin(&text);
    println!("{latin}");
}

fn pig_latin(text: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        if is_vowel(&word.chars().next().expect("word must not be empty")) {
            words.push(format!("{word}-hay"));
        } else {
            let consonant = word.chars().next().expect("word must not be empty");
            let mut result = String::new();
            let mut iter = word.chars();
            iter.next(); // skip one
            for c in iter {
                result.push(c);
            }
            result.push(consonant);
            result.push_str("-ay");
            words.push(result);
        }
    }
    words.join(" ")
}

fn is_vowel(c: &char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    vowels.contains(c)
}
