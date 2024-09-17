use std::collections::HashMap;
use std::fmt::Debug;

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
}

#[derive(Debug)]
struct Node {
    children: HashMap<char, Option<Node>>,
}

impl Node {
    fn empty() -> Self {
        Node {
            children: HashMap::new(),
        }
    }

    fn new(c: char) -> Self {
        Node {
            children: HashMap::from([(c, None)]),
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
                    let mut n = Node::empty();
                    n.insert(&tail);
                    *e = Some(n);
                }
            })
            .or_insert_with(|| {
                let mut e = Node::empty();
                e.insert(&tail);
                Some(e)
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut words = Trie::new();
        words.insert("cat");
        words.insert("car");
        words.insert("bat");
        assert!(words.root.children.contains_key(&'c'));
        assert!(words.root.children.contains_key(&'b'));
        println!("{:?}", words.root.children);
    }
}
