use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Node<T: Clone + Debug + Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Clone + Debug + Ord,
{
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => match &mut self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(Self::new(value))),
            },
            Ordering::Greater => match &mut self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(Self::new(value))),
            },
            Ordering::Equal => (),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => match &self.left {
                Some(node) => node.contains(value),
                None => false,
            },
            Ordering::Greater => match &self.right {
                Some(node) => node.contains(value),
                None => false,
            },
            Ordering::Equal => true,
        }
    }
}

// TODO
// - contains
// - get_values (in-/post-/pre-order)
// - delete

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut root: Node<usize> = Node::new(4);
        root.insert(2);
        root.insert(1);
        root.insert(3);
        root.insert(6);
        root.insert(5);
        root.insert(7);

        assert_eq!(root.value, 4);
        assert_eq!(root.left.clone().unwrap().value, 2);
        assert_eq!(root.right.clone().unwrap().value, 6);

        let left = root.left.unwrap();
        let right = root.right.unwrap();

        assert_eq!(left.left.unwrap().value, 1);
        assert_eq!(left.right.unwrap().value, 3);
        assert_eq!(right.left.unwrap().value, 5);
        assert_eq!(right.right.unwrap().value, 7);
    }

    #[test]
    fn test_contains() {
        let mut root: Node<usize> = Node::new(4);
        root.insert(2);
        root.insert(1);
        root.insert(3);
        root.insert(6);
        root.insert(5);
        root.insert(7);

        for i in 1..=7 {
            assert!(root.contains(&i));
        }
        assert!(!root.contains(&0));
        assert!(!root.contains(&8));
    }
}
