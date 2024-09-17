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

    fn find_by_prefix(&self, prefix: &str) -> Option<Node> {
        let mut chars = prefix.chars();
        match chars.next() {
            Some(c) => match self.children.get(&c) {
                Some(child) => match child {
                    Some(node) => node.find_by_prefix(&String::from_iter(chars)),
                    None => None,
                },
                None => None,
            },
            None => Some(self.clone()),
        }
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

    // TODO: completions: prefix + collected suffixes from sub-nodes
}
