use rand::Rng;
use std::collections::HashMap;

pub fn build_markov_chain(text: &str, prefix_size: usize, length: usize) -> String {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    let prefix_suffix_map = build_prefix_suffix_map(&words, prefix_size);
    let mut w1 = words[0].clone();
    let mut w2 = words[1].clone();
    let mut output: Vec<String> = vec![w1.clone(), w2.clone()];
    for _ in 0..length {
        let prefix = vec![w1.clone(), w2.clone()];
        let suffixes = match prefix_suffix_map.get(&prefix) {
            Some(value) => value,
            None => break,
        };
        let j = rand::thread_rng().gen_range(0..suffixes.len());
        let w3 = suffixes[j].clone();
        w1 = w2.clone();
        w2 = w3.clone();
        output.push(w3.clone());
    }
    output.join(" ")
}

fn build_prefix_suffix_map(
    text: &[String],
    prefix_size: usize,
) -> HashMap<Vec<String>, Vec<String>> {
    let mut map: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    for (i, word) in text.iter().enumerate() {
        if i < prefix_size {
            continue;
        }
        let mut prefix: Vec<String> = Vec::new();
        for j in (1..=prefix_size).rev() {
            prefix.push(text[i - j].clone());
        }
        map.entry(prefix)
            .and_modify(|v| v.push(word.clone()))
            .or_insert(vec![word.clone()]);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_prefix_suffix_map() {
        let text = "
        Show your flowcharts and conceal your tables and I will be mystified.
        Show your tables and your flowcharts will be obvious.
        "
        .to_string();
        let words: Vec<String> = text
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let expected: HashMap<Vec<String>, Vec<String>> = HashMap::from([
            (
                vec!["Show".into(), "your".into()],
                vec!["flowcharts".into(), "tables".into()],
            ),
            (
                vec!["your".into(), "flowcharts".into()],
                vec!["and".into(), "will".into()],
            ),
            (
                vec!["flowcharts".into(), "and".into()],
                vec!["conceal".into()],
            ),
            (vec!["and".into(), "conceal".into()], vec!["your".into()]),
            (vec!["conceal".into(), "your".into()], vec!["tables".into()]),
            (
                vec!["your".into(), "tables".into()],
                vec!["and".into(), "and".into()], // NOTE: redundant? use set?
            ),
            (
                vec!["tables".into(), "and".into()],
                vec!["I".into(), "your".into()],
            ),
            (vec!["and".into(), "I".into()], vec!["will".into()]),
            (vec!["I".into(), "will".into()], vec!["be".into()]),
            (
                vec!["will".into(), "be".into()],
                vec!["mystified.".into(), "obvious.".into()],
            ),
            (vec!["be".into(), "mystified.".into()], vec!["Show".into()]),
            (
                vec!["mystified.".into(), "Show".into()],
                vec!["your".into()],
            ),
            (vec!["and".into(), "your".into()], vec!["flowcharts".into()]),
            (vec!["flowcharts".into(), "will".into()], vec!["be".into()]),
        ]);
        let actual = build_prefix_suffix_map(&words, 2);
        assert_eq!(actual, expected);
    }
}
