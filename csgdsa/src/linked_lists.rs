#[derive(Clone)]
pub struct Node<T: Clone + Eq> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Clone + Eq,
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
    fn insert_at_index() {
        let mut list: Node<usize> = Node::new(0);
        list.append(10);
        list.append(20);
        list.append(30);
        list.insert_after(5, 0); // 0, 5, 10, 20, 30
        list.insert_after(15, 2); // 0, 5, 10, 15, 20, 30
        list.insert_after(25, 4); // 0, 5, 10, 15, 20, 25, 30
        assert_eq!(list.get_values(), vec![0, 5, 10, 15, 20, 25, 30]);
    }
}
