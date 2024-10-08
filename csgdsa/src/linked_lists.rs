use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
pub struct Node<T: Clone + Eq + Display> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Clone + Eq + Display,
{
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }

    pub fn prepend(self, value: T) -> Self {
        Node {
            value,
            next: Some(Box::new(self)),
        }
    }

    pub fn append(&mut self, value: T) {
        let next = Some(Box::new(Node { value, next: None }));
        if self.next.is_none() {
            self.next = next;
            return;
        }
        let mut temp = &mut self.next;
        while let Some(node) = temp {
            if node.next.is_some() {
                temp = &mut node.next;
            } else {
                node.next = next;
                return;
            }
        }
    }

    pub fn get_values(&self) -> Vec<T> {
        let mut values: Vec<T> = vec![self.value.clone()];
        let mut temp = &self.next;
        while let Some(node) = temp {
            values.push(node.value.clone());
            temp = &node.next;
        }
        values
    }

    pub fn nth(&self, n: usize) -> Option<T> {
        if n == 0 {
            return Some(self.value.clone());
        }
        let mut temp = &self.next;
        let mut i = 1;
        while let Some(node) = temp {
            if i == n {
                return Some(node.value.clone());
            }
            temp = &node.next;
            i += 1;
        }
        None
    }

    pub fn search(&self, value: T) -> Option<usize> {
        if self.value == value {
            return Some(0);
        }
        let mut temp = &self.next;
        let mut i = 1;
        while let Some(node) = temp {
            if node.value == value {
                return Some(i);
            }
            temp = &node.next;
            i += 1;
        }
        None
    }

    pub fn insert_after(&mut self, value: T, index: usize) {
        if index == 0 {
            let new = Node {
                value: value.clone(),
                next: self.next.clone(),
            };
            self.next = Some(Box::new(new));
            return;
        }
        let mut temp = &mut self.next;
        let mut i = 0;
        while let Some(ref mut node) = temp {
            if i + 1 == index {
                let new = Node {
                    value: value.clone(),
                    next: node.next.clone(),
                };
                node.next = Some(Box::new(new));
                return;
            } else if i > index {
                panic!("index exceeds length");
            }
            temp = &mut node.next;
            i += 1;
        }
    }

    pub fn delete_at(&mut self, index: usize) {
        if index == 0 {
            panic!("cannot delete at head; set head to next instead");
        }
        if index == 1 {
            if let Some(ref mut next) = self.next {
                self.next = next.next.clone();
                return;
            }
        }
        let mut temp = &mut self.next;
        let mut i = 1;
        while let Some(ref mut node) = temp {
            if i + 1 == index {
                if let Some(ref mut next) = node.next {
                    node.next = next.next.clone();
                }
                return;
            }
            if i > index {
                panic!("index exceeds length");
            }
            temp = &mut node.next;
            i += 1;
        }
    }

    pub fn get_last_value(&self) -> T {
        let mut value = &self.value;
        let mut temp = Some(self);
        while let Some(node) = temp {
            if node.next.is_none() {
                value = &node.value;
                break;
            }
            temp = node.next.as_deref();
        }
        value.clone()
    }

    pub fn reverse(&self) -> Self {
        let mut reversed: Node<T> = Self::new(self.value.clone());
        for v in self.get_values().iter().skip(1) {
            reversed = reversed.prepend(v.clone());
        }
        reversed
    }
}

impl<T> Display for Node<T>
where
    T: Clone + Eq + Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.get_values()
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend() {
        let head: Node<usize> = Node::new(4);
        let head = head.prepend(3);
        let head = head.prepend(2);
        let head = head.prepend(1);
        assert_eq!(head.get_values(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_append() {
        let mut list: Node<usize> = Node::new(1);
        list.append(2);
        list.append(3);
        list.append(4);
        assert_eq!(list.get_values(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_nth() {
        let mut list: Node<&str> = Node::new("one");
        list.append("two");
        list.append("three");
        list.append("four");
        list.append("five");
        assert_eq!(list.nth(0), Some("one"));
        assert_eq!(list.nth(1), Some("two"));
        assert_eq!(list.nth(2), Some("three"));
        assert_eq!(list.nth(3), Some("four"));
        assert_eq!(list.nth(4), Some("five"));
        assert_eq!(list.nth(5), None);
    }

    #[test]
    fn test_search() {
        let mut list: Node<&str> = Node::new("one");
        list.append("two");
        list.append("three");
        list.append("four");
        list.append("five");
        assert_eq!(list.search("one"), Some(0));
        assert_eq!(list.search("two"), Some(1));
        assert_eq!(list.search("three"), Some(2));
        assert_eq!(list.search("four"), Some(3));
        assert_eq!(list.search("five"), Some(4));
        assert_eq!(list.search("six"), None);
    }

    #[test]
    fn test_insert_after_index() {
        let mut list: Node<usize> = Node::new(0);
        list.append(10);
        list.append(20);
        list.append(30);
        list.insert_after(5, 0);
        assert_eq!(list.get_values(), vec![0, 5, 10, 20, 30]);
        list.insert_after(15, 2);
        assert_eq!(list.get_values(), vec![0, 5, 10, 15, 20, 30]);
        list.insert_after(25, 4);
        assert_eq!(list.get_values(), vec![0, 5, 10, 15, 20, 25, 30]);
    }

    #[test]
    fn test_delete_at_index() {
        let mut list: Node<usize> = Node::new(0);
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.delete_at(5);
        assert_eq!(list.get_values(), vec![0, 1, 2, 3, 4]);
        list.delete_at(1);
        assert_eq!(list.get_values(), vec![0, 2, 3, 4]);
        list.delete_at(2);
        assert_eq!(list.get_values(), vec![0, 2, 4]);
    }

    #[test]
    fn test_display_values() {
        let mut list: Node<usize> = Node::new(0);
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        assert_eq!(format!("{}", list), "0, 1, 2, 3, 4, 5");
    }

    #[test]
    fn test_get_last_value() {
        let mut list: Node<usize> = Node::new(0);
        for i in 1..=10 {
            list.append(i);
        }
        assert_eq!(list.get_last_value(), 10);
    }

    #[test]
    fn test_reverse() {
        let mut list: Node<usize> = Node::new(0);
        for i in 1..10 {
            list.append(i);
        }
        let reversed = list.reverse();
        assert_eq!(reversed.get_values(), (0..10).rev().collect::<Vec<_>>());
    }
}
