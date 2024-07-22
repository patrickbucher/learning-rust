use std::cmp::Ordering;

#[derive(Debug)]
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
            Ordering::Less => match self.left {
                Some(ref mut child) => {
                    child.insert(value);
                }
                None => {
                    self.left = Some(Box::new(Node::new(value)));
                }
            },
            _ => match self.right {
                Some(ref mut child) => {
                    child.insert(value);
                }
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                }
            },
        }
    }

    pub fn insert_inplace(self, value: isize) -> Self {
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                Some(left) => {
                    let left_value = left.value;
                    let new = left.insert_inplace(value);
                    if new.get_tree_balance() < -1 {
                        Node {
                            value: self.value,
                            left: Some(Box::new(Node {
                                value: new.left.unwrap().value,
                                left: Some(Box::new(Node {
                                    value,
                                    left: None,
                                    right: None,
                                })),
                                right: Some(Box::new(Node {
                                    value: left_value,
                                    left: None,
                                    right: None,
                                })),
                            })),
                            right: self.right,
                        }
                    } else {
                        Node {
                            value: self.value,
                            left: Some(Box::new(new)),
                            right: self.right,
                        }
                    }
                }
                None => Node {
                    value: self.value,
                    left: Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    })),
                    right: self.right,
                },
            },
            _ => match self.right {
                Some(right) => {
                    let right_value = right.value;
                    let new = right.insert_inplace(value);
                    if new.get_tree_balance() > 1 {
                        Node {
                            value: self.value,
                            left: self.left,
                            right: Some(Box::new(Node {
                                value: new.right.unwrap().value,
                                left: Some(Box::new(Node {
                                    value: right_value,
                                    left: None,
                                    right: None,
                                })),
                                right: Some(Box::new(Node {
                                    value,
                                    left: None,
                                    right: None,
                                })),
                            })),
                        }
                    } else {
                        Node {
                            value: self.value,
                            left: self.left,
                            right: Some(Box::new(new)),
                        }
                    }
                }
                None => Node {
                    value: self.value,
                    left: self.left,
                    right: Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    })),
                },
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

    pub fn get_node_balance(&self) -> isize {
        let mut balance = 0;
        if self.left.is_some() {
            balance -= 1;
        }
        if self.right.is_some() {
            balance += 1;
        }
        balance
    }

    pub fn get_tree_balance(&self) -> isize {
        let left_balance = match &self.left {
            Some(node) => node.get_tree_balance(),
            None => 0,
        };
        let right_balance = match &self.right {
            Some(node) => node.get_tree_balance(),
            None => 0,
        };
        left_balance + self.get_node_balance() + right_balance
    }

    pub fn get_values(&self) -> Vec<isize> {
        let mut values = Vec::new();
        values.extend(match &self.left {
            Some(node) => node.get_values(),
            None => Vec::new(),
        });
        values.push(self.value);
        values.extend(match &self.right {
            Some(node) => node.get_values(),
            None => Vec::new(),
        });
        values
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
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
        assert_eq!(tree.get_node_balance(), 0);
        let left = tree.left.unwrap();
        let right = tree.right.unwrap();
        assert_eq!(left.value, 3);
        assert_eq!(left.get_node_balance(), 0);
        assert_eq!(right.value, 7);
        assert_eq!(right.get_node_balance(), 0);
        let ll = left.left.unwrap();
        let lr = left.right.unwrap();
        let rl = right.left.unwrap();
        let rr = right.right.unwrap();
        assert_eq!(ll.value, 2);
        assert_eq!(ll.get_node_balance(), 0);
        assert_eq!(lr.value, 4);
        assert_eq!(lr.get_node_balance(), 0);
        assert_eq!(rl.value, 6);
        assert_eq!(rl.get_node_balance(), 0);
        assert_eq!(rr.value, 8);
        assert_eq!(rr.get_node_balance(), 0);
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

    #[test]
    fn node_balance() {
        let mut tree = Node::new(5);
        tree.insert(7);
        tree.insert(9);
        assert_eq!(tree.get_tree_balance(), 2);
        tree.insert(3);
        assert_eq!(tree.get_tree_balance(), 1);
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.get_tree_balance(), -1);
    }

    #[test]
    fn get_values_inserted_in_order() {
        let mut tree = Node::new(4);
        for value in [2, 6, 1, 3, 5, 7] {
            tree.insert(value);
        }
        let expected = vec![1, 2, 3, 4, 5, 6, 7];
        let actual = tree.get_values();
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_values_inserted_ascendingly() {
        let mut tree = Node::new(1);
        assert_eq!(tree.get_values(), vec![1]);
        tree.insert(2);
        assert_eq!(tree.get_values(), vec![1, 2]);
        tree.insert(3);
        assert_eq!(tree.get_values(), vec![1, 2, 3]);
        tree.insert(4);
        assert_eq!(tree.get_values(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn insert_numbers_inplace() {
        let mut tree = Node::new(5);
        for value in [3, 7, 2, 4, 6, 8] {
            tree = tree.insert_inplace(value);
        }
        assert_eq!(tree.get_values(), (2..=8).collect::<Vec<isize>>());
    }

    #[test]
    fn insert_unbalanced_inplace_left() {
        let mut tree = Node::new(9);
        for value in (1..=8).rev() {
            tree = tree.insert_inplace(value);
        }
        assert_eq!(tree.get_values(), (1..=9).collect::<Vec<isize>>());
    }

    #[test]
    fn insert_unbalanced_inplace_right() {
        let mut tree = Node::new(1);
        for value in 2..=9 {
            tree = tree.insert_inplace(value);
        }
        assert_eq!(tree.get_values(), (1..=9).collect::<Vec<isize>>());
    }
}
