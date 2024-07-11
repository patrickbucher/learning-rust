use std::cmp::Ordering;

pub struct Node {
    value: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: isize) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: isize) {
        match value.cmp(&self.value) {
            Ordering::Less => match &mut self.left {
                Some(ref mut child) => child.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            },
            _ => match &mut self.right {
                Some(ref mut child) => child.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            },
        }
    }

    pub fn contains(&self, value: isize) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => match &self.left {
                Some(child) => child.contains(value),
                None => false,
            },
            Ordering::Greater => match &self.right {
                Some(child) => child.contains(value),
                None => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_numbers() {
        let mut tree = Node::new(5);
        for value in [3, 7, 2, 4, 6, 8] {
            tree.insert(value);
        }
        assert_eq!(tree.value, 5);
        let left = tree.left.unwrap();
        let right = tree.right.unwrap();
        assert_eq!(left.value, 3);
        assert_eq!(right.value, 7);
        let ll = left.left.unwrap();
        let lr = left.right.unwrap();
        let rl = right.left.unwrap();
        let rr = right.right.unwrap();
        assert_eq!(ll.value, 2);
        assert_eq!(lr.value, 4);
        assert_eq!(rl.value, 6);
        assert_eq!(rr.value, 8);
    }

    #[test]
    fn contains_numbers() {
        let mut tree = Node::new(5);
        let values = [3, 7, 2, 4, 6, 8];
        for value in values {
            tree.insert(value);
        }
        for value in values {
            assert!(tree.contains(value));
        }
        assert!(!tree.contains(1));
        assert!(!tree.contains(9));
    }
}
