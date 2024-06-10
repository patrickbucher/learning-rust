use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap();
    let file = args.next().expect(&format!("usage: {bin} FILE"));
    let content = fs::read_to_string(&file)
        .map_err(|e| format!("read {file}: {e}"))
        .unwrap();
    let content: String = content.replace("\n", " ");
    let words: Vec<_> = content.trim().split(" ").collect();
    let mut freqs: HashMap<&str, usize> = HashMap::new();
    for word in words {
        freqs.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut freqs: Vec<(&str, usize)> = freqs.into_iter().map(|(k, v)| (k, v)).collect();
    freqs.sort_by(|(_, l), (_, r)| r.cmp(l));
    for (word, freq) in freqs {
        println!("{word:<20}{}", "*".repeat(freq));
    }
}
