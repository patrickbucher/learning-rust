use std::rc::Rc;
use std::cell::RefCell;

pub struct Node<'a> {
    value: i32,
    next: Rc<RefCell<List<'a>>>,
}

impl<'a> Node<'a> {
    pub fn next_value(&self) -> Option<i32> {
        let rca = Rc::clone(&self.next);
        let rfa = rca.borrow();

        if let List::Some(node) = *rfa {
            Some(node.value)
        } else {
            None
        }
    }
}

pub enum List<'a> {
    Some(&'a Node<'a>),
    None,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nothing() {
        let a = Node{
            value: 1,
            next: Rc::new(RefCell::new(List::None)),
        };
        let b = Node{
            value: 2,
            next: Rc::new(RefCell::new(List::None)),
        };

        let rca = Rc::clone(&a.next);
        let rcb = Rc::clone(&b.next);

        let mut rfa = rca.borrow_mut();
        let mut rfb = rcb.borrow_mut();

        *rfa = List::Some(&b);
        *rfb = List::Some(&a);

        std::mem::drop(rfa);
        std::mem::drop(rfb);

        assert_eq!(a.next_value(), Some(2));
        assert_eq!(b.next_value(), Some(1));
    }
}
