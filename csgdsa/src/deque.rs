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

    pub fn enqueue(&mut self, value: T) {}

    pub fn dequeue(&mut self) -> Option<T> {
        None
    }
}

#[cfg(test)]
mod tests {
    fn test_enqueue_dequeue() {
        let mut deque: Deque<usize> = Deque::new();
        assert!(deque.is_empty());

        deque.enqueue(3);
        assert!(!deque.is_empty());

        assert_eq!(dequeue.dequeue(), Some(3));
        assert!(deque.is_empty());
        assert_eq!(dequeue.dequeue(), None);

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
