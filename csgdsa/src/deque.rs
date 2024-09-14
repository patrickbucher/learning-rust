use std::cell::RefCell;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

pub struct Node<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Deque<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Default for Deque<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deque<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn enqueue(&mut self, value: T) {
        match &self.head {
            Some(head) => {
                let node = Node {
                    value,
                    next: Some(Rc::clone(head)),
                    prev: None,
                };
                let node_ref = Rc::new(RefCell::new(node));
                head.borrow_mut().prev = Some(Rc::clone(&node_ref));
                self.head = Some(Rc::clone(&node_ref));
            }
            None => {
                let node = Node {
                    value,
                    next: None,
                    prev: None,
                };
                let rc = Rc::new(RefCell::new(node));
                self.head = Some(Rc::clone(&rc));
                self.tail = Some(Rc::clone(&rc));
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let mut value;
        (self.tail, value) = match &self.tail {
            Some(cell) => {
                let node = cell.borrow_mut();
                value = Some(node.value.clone());
                match &node.prev {
                    Some(prev) => {
                        let mut c = prev.borrow_mut();
                        c.next = None;
                        (Some(prev.clone()), value)
                    }
                    None => {
                        self.head = None;
                        (None, value)
                    }
                }
            }
            None => (None, None),
        };
        value
    }

    pub fn get_values(&self) -> Vec<T> {
        let mut values = Vec::new();
        let mut temp = self.head.clone();
        while let Some(node) = temp {
            let borrowed = node.borrow();
            values.push(borrowed.value.clone());
            temp = borrowed.next.clone();
        }
        values
    }
}

impl<T> Display for Deque<T>
where
    T: Clone + Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let values = self
            .get_values()
            .iter()
            .rev()
            .map(|v| format!("{v}"))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{values}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut deque: Deque<usize> = Deque::new();
        assert!(deque.is_empty());
        assert_eq!(deque.get_values(), Vec::new());

        deque.enqueue(3);
        assert_eq!(deque.get_values(), vec![3]);
        assert!(!deque.is_empty());

        assert_eq!(deque.dequeue(), Some(3));
        assert!(deque.is_empty());
        assert_eq!(deque.get_values(), Vec::new());
        assert_eq!(deque.dequeue(), None);

        deque.enqueue(7);
        deque.enqueue(2);
        deque.enqueue(5);
        assert!(!deque.is_empty());
        assert_eq!(deque.get_values(), vec![5, 2, 7]);

        assert_eq!(deque.dequeue(), Some(7));
        assert_eq!(deque.get_values(), vec![5, 2]);
        assert_eq!(deque.dequeue(), Some(2));
        assert!(!deque.is_empty());
        assert_eq!(deque.get_values(), vec![5]);
        assert_eq!(deque.dequeue(), Some(5));
        assert!(deque.is_empty());
        assert_eq!(deque.get_values(), Vec::new());
        assert_eq!(deque.dequeue(), None);
    }

    #[test]
    fn test_display_reverse() {
        let mut deque: Deque<usize> = Deque::new();
        for i in 0..5 {
            deque.enqueue(i);
        }
        assert_eq!(format!("{}", deque), "0, 1, 2, 3, 4");
    }
}
