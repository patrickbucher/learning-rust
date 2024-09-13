use std::cell::RefCell;
use std::rc::Rc;

pub struct Deque<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Node<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
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
        if self.head.is_none() {
            let mut node = Node {
                value,
                next: None,
                prev: None,
            };
            let rc = Rc::new(RefCell::new(node));
            self.head = Some(Rc::clone(&rc));
            self.tail = Some(Rc::clone(&rc));
        } else if self.head.is_some() {
            let mut head = self.head.take().unwrap();
            let mut node = Node {
                value,
                next: None,
                prev: Some(Rc::clone(&head)),
            };
            node.next = Some(Rc::clone(&head));
            self.head = Some(Rc::new(RefCell::new(node)));
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let mut new_tail;
        let mut value: Option<T>;
        match &self.tail {
            Some(cell) => {
                let node = cell.borrow_mut();
                value = Some(node.value.clone());
                match &node.prev {
                    Some(prev) => {
                        let mut c = prev.borrow_mut();
                        c.next = None;
                        new_tail = Some(prev.clone());
                    }
                    None => {
                        new_tail = None;
                        self.head = None;
                    }
                }
            }
            None => {
                value = None;
                new_tail = None;
            }
        }
        self.tail = new_tail.clone();
        return value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut deque: Deque<usize> = Deque::new();
        assert!(deque.is_empty());

        deque.enqueue(3);
        assert!(!deque.is_empty());

        assert_eq!(deque.dequeue(), Some(3));
        assert!(deque.is_empty());
        assert_eq!(deque.dequeue(), None);

        deque.enqueue(7);
        deque.enqueue(2);
        deque.enqueue(5);
        assert!(!deque.is_empty());

        assert_eq!(deque.dequeue(), Some(7));
        assert_eq!(deque.dequeue(), Some(2));
        assert!(!deque.is_empty());
        assert_eq!(deque.dequeue(), Some(5));
        assert!(deque.is_empty());
        assert_eq!(deque.dequeue(), None);
    }
}
