use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};

const EOW: char = '*';

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node {
                children: HashMap::new(),
            },
        }
    }

    pub fn insert(&mut self, word: &str) {
        self.root.insert(word);
    }

    pub fn autocomplete(&self, prefix: &str) -> Vec<(String, String)> {
        let mut result = Vec::new();
        let node = self.find_by_prefix(prefix);
        if let Some(node) = node {
            for suffix in node.find_words() {
                result.push((String::from(prefix), suffix));
            }
        }
        result
    }

    pub fn autocorrect(&self, word: &str) -> Option<Vec<String>> {
        if let Some(node) = self.find_by_prefix(word) {
            if node.children.contains_key(&EOW) {
                return None; // correct word
            }
        }
        let mut prefix: Vec<char> = word.chars().collect();
        let mut suggestions: Vec<String> = Vec::new();
        while !prefix.is_empty() && suggestions.is_empty() {
            prefix = prefix[0..prefix.len() - 1].to_vec();
            suggestions = self
                .autocomplete(&String::from_iter(prefix.clone()))
                .iter()
                .map(|(p, s)| format!("{p}{s}"))
                .collect();
        }
        suggestions.sort();
        Some(suggestions)
    }

    fn find_by_prefix(&self, prefix: &str) -> Option<Node> {
        self.root.find_by_prefix(prefix)
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct Node {
    children: HashMap<char, Option<Node>>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut chars = word.chars();
        let head: char = match chars.next() {
            Some(c) => c,
            None => {
                self.children.insert(EOW, None);
                return;
            }
        };
        let tail = String::from_iter(chars);
        self.children
            .entry(head)
            .and_modify(|e| {
                if let Some(node) = e {
                    node.insert(&tail);
                } else {
                    let mut n = Node::new();
                    n.insert(&tail);
                    *e = Some(n);
                }
            })
            .or_insert_with(|| {
                let mut e = Node::new();
                e.insert(&tail);
                Some(e)
            });
    }

    fn find_words(&self) -> Vec<String> {
        self.find_words_by_prefix("")
    }

    fn find_words_by_prefix(&self, prefix: &str) -> Vec<String> {
        let mut words = Vec::new();
        for (c, child) in self.children.clone() {
            match child {
                Some(node) => {
                    let prefix = &format!("{prefix}{c}");
                    let parts = &node.find_words_by_prefix(prefix);
                    for part in parts {
                        words.push(part.clone());
                    }
                }
                None => words.push(String::from(prefix)),
            }
        }
        words
    }

    fn find_by_prefix(&self, prefix: &str) -> Option<Node> {
        let mut chars = prefix.chars();
        match chars.next() {
            Some(c) => match self.children.get(&c) {
                Some(Some(node)) => node.find_by_prefix(&String::from_iter(chars)),
                _ => None,
            },
            None => Some(self.clone()),
        }
    }

    fn collect_keys(&self, level: usize, acc: &mut HashMap<usize, Vec<String>>) {
        let mut keys: Vec<String> = Vec::new();
        for (key, _) in self.children.clone() {
            keys.push(String::from(key));
        }
        keys.sort();
        let mut output = String::new();
        output.push('{');
        output.push_str(&keys.join(",").to_string());
        output.push('}');
        acc.entry(level)
            .and_modify(|e| e.push(output.clone()))
            .or_insert(vec![output]);
        for node in self.children.values().flatten() {
            node.collect_keys(level + 1, acc);
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut keys = HashMap::new();
        self.collect_keys(0, &mut keys);
        let mut lines: Vec<String> = Vec::new();
        let mut levels: Vec<&usize> = keys.keys().collect();
        levels.sort();
        for level in levels {
            if let Some(outputs) = keys.get(level) {
                let mut outputs = outputs.clone();
                outputs.sort();
                lines.push(outputs.join(","));
            }
        }
        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read;

    #[test]
    fn test_insert() {
        let mut words = Trie::new();
        words.insert("cat");
        words.insert("car");
        words.insert("bat");

        let children = words.root.children.clone();
        assert!(children.contains_key(&'c'));
        assert!(children.contains_key(&'b'));

        let c_children = children.get(&'c').unwrap().clone().unwrap().children;
        assert!(c_children.contains_key(&'a'));

        let b_children = children.get(&'b').unwrap().clone().unwrap().children;
        assert!(b_children.contains_key(&'a'));

        let ca_children = c_children.get(&'a').unwrap().clone().unwrap().children;
        assert!(ca_children.contains_key(&'t'));
        assert!(ca_children.contains_key(&'r'));

        let ba_children = b_children.get(&'a').unwrap().clone().unwrap().children;
        assert!(ba_children.contains_key(&'t'));

        let cat_children = ca_children.get(&'t').unwrap().clone().unwrap().children;
        assert!(cat_children.contains_key(&EOW));

        let car_children = ca_children.get(&'r').unwrap().clone().unwrap().children;
        assert!(car_children.contains_key(&EOW));

        let bat_children = ba_children.get(&'t').unwrap().clone().unwrap().children;
        assert!(bat_children.contains_key(&EOW));
    }

    #[test]
    fn test_find_by_prefix() {
        let mut words = Trie::new();
        words.insert("car");
        words.insert("cat");
        words.insert("catapult");
        words.insert("catnip");
        words.insert("catwalk");
        words.insert("cattle");
        words.insert("cauldron");

        let c_node = words.find_by_prefix("c").unwrap();
        let children = c_node.children;
        assert!(children.contains_key(&'a'));

        let ca_node = words.find_by_prefix("ca").unwrap();
        let children = ca_node.children;
        assert!(children.contains_key(&'r'));
        assert!(children.contains_key(&'t'));
        assert!(children.contains_key(&'u'));

        let car_node = words.find_by_prefix("car").unwrap();
        let children = car_node.children;
        assert!(children.contains_key(&EOW));

        let cat_node = words.find_by_prefix("cat").unwrap();
        let children = cat_node.children;
        assert!(children.contains_key(&EOW));
        assert!(children.contains_key(&'a'));
        assert!(children.contains_key(&'n'));
        assert!(children.contains_key(&'w'));
        assert!(children.contains_key(&'t'));
    }

    #[test]
    fn test_find_words() {
        let mut words = Trie::new();
        let mut dict = vec!["ant", "ale", "ace", "bat", "bar", "boy"];
        for word in &dict {
            words.insert(word);
        }

        let root_node = words.root.clone();
        let mut actual = root_node.find_words();
        dict.sort();
        actual.sort();
        assert_eq!(actual, dict);

        let a_node = words.find_by_prefix("a").unwrap();
        let mut expected = vec!["nt", "le", "ce"];
        let mut actual = a_node.find_words();
        expected.sort();
        actual.sort();
        assert_eq!(actual, expected);

        let b_node = words.find_by_prefix("b").unwrap();
        let mut expected = vec!["at", "ar", "oy"];
        let mut actual = b_node.find_words();
        expected.sort();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_keys() {
        let mut words = Trie::new();
        for word in [
            "get", "go", "got", "gotten", "hall", "ham", "hammer", "hill", "zebra",
        ] {
            words.insert(word);
        }
        let expected = "
{g,h,z}
{a,i},{e,o},{e}
{*,t},{b},{l,m},{l},{t}
{*,m},{*,t},{*},{l},{l},{r}
{*},{*},{a},{e},{e}
{*},{n},{r}
{*},{*}"
            .trim();
        let actual = format!("{}", words.root);
        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn test_autocomplete() {
        let mut words = Trie::new();
        for line in get_dict() {
            words.insert(&line);
        }
        let completions = words.autocomplete("aban");
        let expected = vec!["don", "doned", "doning", "donment", "donment's", "dons"];
        let expected: Vec<(String, String)> = expected
            .iter()
            .map(|s| (String::from("aban"), String::from(*s)))
            .collect();
        for result in expected {
            assert!(completions.contains(&result));
        }
    }

    #[test]
    #[ignore]
    fn test_autocorrect() {
        let mut words = Trie::new();
        for line in get_dict() {
            words.insert(&line);
        }

        let actual = words.autocorrect("catnar");
        let expected: Vec<String> =
            vec!["catnap", "catnap's", "catnapped", "catnapping", "catnaps"]
                .iter()
                .map(|s| String::from(*s))
                .collect();
        assert_eq!(actual, Some(expected));

        let actual = words.autocorrect("membrax");
        let expected: Vec<String> = vec!["membrane", "membrane's", "membranes", "membranous"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        assert_eq!(actual, Some(expected));

        let actual = words.autocorrect("memento");
        assert_eq!(actual, None); // correct word
    }

    fn get_dict() -> Vec<String> {
        let dict_path = "/usr/share/dict/words";
        let data = match read(dict_path) {
            Ok(data) => data,
            Err(err) => panic!("reading file: {err}"),
        };
        let text = match String::from_utf8(data) {
            Ok(text) => text,
            Err(err) => panic!("decoding utf8: {err}"),
        };
        text.split('\n')
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
    }
}
