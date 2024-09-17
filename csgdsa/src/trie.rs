use std::collections::HashMap;

const EOW: char = '*';

pub struct Trie {
    root: Node,
}

struct Node {
    children: HashMap<char, Node>,
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
}

impl Node {
    fn empty() -> Self {
        Node {
            children: HashMap::new(),
        }
    }

    fn new(c: char) -> Self {
        Node {
            children: HashMap::from([(c, Self::empty())]),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut chars = word.chars();
        let head: char = match chars.next() {
            Some(c) => c,
            None => return,
        };
        let tail = String::from_iter(chars);
        self.children
            .entry(head)
            .and_modify(|e| {
                e.insert(&tail);
            })
            .or_insert_with(|| {
                let mut e = Node::new(head);
                e.insert(&tail);
                e
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_insert() {
        let mut words = Trie::new();
        words.insert("cat");
        assert!(words.root.children.contains_key(&'c'));
    }
}
