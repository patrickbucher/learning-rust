use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T>
where
    T: Copy,
{
    prev: Rc<RefCell<List<T>>>,
    next: Rc<RefCell<List<T>>>,
    pub value: T,
}

pub enum List<T>
where
    T: Copy,
{
    Some(Node<T>),
    None,
}

impl<T> Node<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Node<T> {
        Node {
            prev: Rc::new(RefCell::new(List::None)),
            next: Rc::new(RefCell::new(List::None)),
            value,
        }
    }

    pub fn list(value: T) -> List<T> {
        List::Some(Self::new(value))
    }

    pub fn connect(&mut self, other: Node<T>) {
        let rc = Rc::clone(&self.next);
        let mut rf = rc.borrow_mut();
        if let List::Some(ref mut old_next) = *rf {
            old_next.prev = Rc::new(RefCell::new(List::None));
        }
        self.next = Rc::new(RefCell::new(List::Some(other)));
    }

    pub fn values(&self) -> Vec<T> {
        let mut vec = vec![self.value];
        let mut rc: Rc<RefCell<List<T>>> = Rc::clone(&self.next);
        loop {
            let rf = rc.borrow();
            match *rf {
                List::Some(ref next) => {
                    vec.push(next.value);
                    break; // TODO: advance rc
                }
                List::None => {
                    break;
                }
            }
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_nodes() {
        let mut head = Node::new(1);
        head.connect(Node::new(2));
        if let List::Some(ref node) = *Rc::clone(&head.next).borrow() {
            assert_eq!(node.value, 2);
        } else {
            panic!("mismatch after connect");
        }
        assert_eq!(head.values(), vec![1, 2]);
    }
}
