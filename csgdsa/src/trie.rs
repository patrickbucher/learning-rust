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
}
