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

impl<T> Deque<T> where T: Clone {
    pub fn enqueue(&mut self, value: T) {
    }

    pub fn dequeue(&mut self) -> Option<T> {
        None
    }
}
