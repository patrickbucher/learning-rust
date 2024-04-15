use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Item(Node),
    Nil,
}

pub struct Node {
    value: i32,
    prev: Rc<RefCell<List>>,
    next: Rc<RefCell<List>>,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value,
            prev: Rc::new(RefCell::new(List::Nil)),
            next: Rc::new(RefCell::new(List::Nil)),
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn prepend(self, value: i32) -> Self {
        Self::new(value)
    }
}
