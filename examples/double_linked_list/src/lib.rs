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
        let mut new_node = Self::new(value);
        new_node.next = Rc::new(RefCell::new(List::Item(self)));
        new_node
    }

    pub fn values(self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = Rc::new(RefCell::new(List::Item(self)));
        loop {
            let rc = Rc::clone(&current);
            let rf = rc.borrow();
            if let List::Item(ref node) = *rf {
                result.push(node.value);
                current = Rc::clone(&node.next);
            } else {
                break;
            }
        }
        result
    }
}
