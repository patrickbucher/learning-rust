use crate::middle_out::middle_out;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Node<T: Clone + Debug + Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub enum Traversal {
    InOrder,
    PreOrder,
    PostOrder,
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

    pub fn get_values(&self, order: &Traversal) -> Vec<T> {
        let left = match &self.left {
            Some(node) => node.get_values(order),
            None => Vec::new(),
        };
        let middle = vec![self.value.clone()];
        let right = match &self.right {
            Some(node) => node.get_values(order),
            None => Vec::new(),
        };
        match order {
            Traversal::InOrder => [left, middle, right].concat(),
            Traversal::PreOrder => [middle, left, right].concat(),
            Traversal::PostOrder => [left, right, middle].concat(),
        }
    }

    pub fn delete(&self, value: &T) -> Option<Self> {
        let values = self.get_values(&Traversal::InOrder);
        let values: Vec<T> = values.into_iter().filter(|v| v != value).collect();
        let values = middle_out(&values);
        if values.is_empty() {
            None
        } else {
            let mut root = Self::new(values[0].clone());
            for v in &values[1..] {
                root.insert(v.clone());
            }
            Some(root)
        }
    }

    pub fn find_max(&self) -> T {
        match &self.right {
            Some(node) => node.find_max(),
            None => self.value.clone(),
        }
    }
}

// TODO
// - get_values (post-/pre-order)

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

    #[test]
    fn test_get_values() {
        let mut root: Node<usize> = Node::new(4);
        root.insert(2);
        root.insert(1);
        root.insert(3);
        root.insert(6);
        root.insert(5);
        root.insert(7);

        let expected: Vec<usize> = (1..=7).collect();
        let actual = root.get_values(&Traversal::InOrder);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_delete_leaf() {
        let tree = create_demo_tree();
        let tree = tree.delete(&4).unwrap();
        assert_eq!(
            tree.get_values(&Traversal::InOrder),
            vec![10, 11, 25, 30, 33, 40, 50, 52, 56, 61, 75, 82, 89, 95]
        );
    }

    #[test]
    fn test_delete_middle_node() {
        let tree = create_demo_tree();
        let tree = tree.delete(&25).unwrap();
        assert_eq!(
            tree.get_values(&Traversal::InOrder),
            vec![4, 10, 11, 30, 33, 40, 50, 52, 56, 61, 75, 82, 89, 95]
        );
    }

    #[test]
    fn test_delete_root() {
        let tree = create_demo_tree();
        assert_eq!(tree.value, 50);
        let tree = tree.delete(&50).unwrap();
        assert_eq!(
            tree.get_values(&Traversal::InOrder),
            vec![4, 10, 11, 25, 30, 33, 40, 52, 56, 61, 75, 82, 89, 95]
        );
    }

    #[test]
    fn test_find_max() {
        let tree = create_demo_tree();
        assert_eq!(tree.find_max(), 95);
    }

    #[test]
    fn test_get_values_in_order() {
        let tree = create_book_tree();
        let expected = vec![
            "Alice in Wonderland",
            "Great Expectations",
            "Lord of the Flies",
            "Moby Dick",
            "Pride and Prejudice",
            "Robinson Crusoe",
            "The Odyssey",
        ];
        let actual = tree.get_values(&Traversal::InOrder);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_values_pre_order() {
        let tree = create_book_tree();
        let expected = vec![
            "Moby Dick",
            "Great Expectations",
            "Alice in Wonderland",
            "Lord of the Flies",
            "Robinson Crusoe",
            "Pride and Prejudice",
            "The Odyssey",
        ];
        let actual = tree.get_values(&Traversal::PreOrder);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_values_post_order() {
        let tree = create_book_tree();
        let expected = vec![
            "Alice in Wonderland",
            "Lord of the Flies",
            "Great Expectations",
            "Pride and Prejudice",
            "The Odyssey",
            "Robinson Crusoe",
            "Moby Dick",
        ];
        let actual = tree.get_values(&Traversal::PostOrder);
        assert_eq!(actual, expected);
    }

    fn create_demo_tree() -> Node<usize> {
        let mut root: Node<usize> = Node::new(50);
        root.insert(25);
        root.insert(75);
        root.insert(10);
        root.insert(33);
        root.insert(56);
        root.insert(89);
        root.insert(4);
        root.insert(11);
        root.insert(30);
        root.insert(40);
        root.insert(52);
        root.insert(61);
        root.insert(82);
        root.insert(95);
        assert_eq!(
            root.get_values(&Traversal::InOrder),
            vec![4, 10, 11, 25, 30, 33, 40, 50, 52, 56, 61, 75, 82, 89, 95]
        );
        root
    }

    fn create_book_tree() -> Node<&'static str> {
        let mut root: Node<&str> = Node::new("Moby Dick");
        root.insert("Great Expectations");
        root.insert("Robinson Crusoe");
        root.insert("Alice in Wonderland");
        root.insert("Lord of the Flies");
        root.insert("Pride and Prejudice");
        root.insert("The Odyssey");
        root
    }
}
