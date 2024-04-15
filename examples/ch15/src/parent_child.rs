use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn strong_and_weak_count() {
        let left = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        });
        let right = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        });
        let root = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&left), Rc::clone(&right)]),
        });

        *left.parent.borrow_mut() = Rc::downgrade(&root);
        *right.parent.borrow_mut() = Rc::downgrade(&root);

        assert_eq!(Rc::strong_count(&root), 1);
        assert_eq!(Rc::weak_count(&root), 2);

        let parent = left.parent.borrow().upgrade();
        if let Some(node) = parent {
            assert_eq!(node.value, 2);
        } else {
            panic!("unable to access parent of left");
        }

        std::mem::drop(root);

        let parent = right.parent.borrow().upgrade();
        if let Some(_) = parent {
            panic!("root should have been disposed of");
        }
    }
}
