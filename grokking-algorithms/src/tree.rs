use std::cmp::Ordering;

pub struct Node {
    value: isize,
    balance: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: isize) -> Self {
        Node {
            value,
            balance: 0,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: isize) {
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                Some(ref mut child) => {
                    child.insert(value);
                    if child.get_total_balance() < -1 {
                        // TODO: fix bogus code
                        // self.parent = self.left
                        // self.right = self
                        // self.left = None
                        /*
                        let mut left: &mut Option<Box<Node>> = &mut child.left;
                        let mut left_inner: Box<Node> = left.take().unwrap();
                        let left_left: Option<Box<Node>> = left_inner.left.take();
                        self.left = left_left;
                        self.right = Some(left_inner);
                        */
                    }
                }
                None => {
                    self.left = Some(Box::new(Node::new(value)));
                    self.balance -= 1;
                }
            },
            _ => match self.right {
                Some(ref mut child) => {
                    child.insert(value);
                    if child.get_total_balance() > 1 {
                        // TODO: fix bogus code
                        // self.parent = self.right
                        // self.left = self
                        // self.right = None
                        /*
                        let mut right: &mut Option<Box<Node>> = &mut child.right;
                        let mut right_inner: Box<Node> = right.take().unwrap();
                        let right_right: Option<Box<Node>> = right_inner.right.take();
                        self.right = right_right;
                        self.left = Some(right_inner);
                        */
                    }
                }
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                    self.balance += 1;
                }
            },
        }
    }

    pub fn insert_balanced(self, value: isize) -> Self {
        let own = self.value;
        match value.cmp(&own) {
            Ordering::Less => {},
            _ => {},
        }
        self
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

    pub fn get_total_balance(&self) -> isize {
        let left_balance = match &self.left {
            Some(node) => node.get_total_balance(),
            None => 0,
        };
        let right_balance = match &self.right {
            Some(node) => node.get_total_balance(),
            None => 0,
        };
        left_balance + self.balance + right_balance
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
        assert_eq!(tree.balance, 0);
        let left = tree.left.unwrap();
        let right = tree.right.unwrap();
        assert_eq!(left.value, 3);
        assert_eq!(left.balance, 0);
        assert_eq!(right.value, 7);
        assert_eq!(right.balance, 0);
        let ll = left.left.unwrap();
        let lr = left.right.unwrap();
        let rl = right.left.unwrap();
        let rr = right.right.unwrap();
        assert_eq!(ll.value, 2);
        assert_eq!(ll.balance, 0);
        assert_eq!(lr.value, 4);
        assert_eq!(lr.balance, 0);
        assert_eq!(rl.value, 6);
        assert_eq!(rl.balance, 0);
        assert_eq!(rr.value, 8);
        assert_eq!(rr.balance, 0);
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
        assert_eq!(tree.get_total_balance(), 2);
        tree.insert(3);
        assert_eq!(tree.get_total_balance(), 1);
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.get_total_balance(), -1);
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
}
