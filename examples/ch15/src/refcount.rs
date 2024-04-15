use std::rc::Rc;

pub enum Node {
    Some(i32, Rc<Node>),
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_heads() {
        let tail = Rc::new(Node::Some(
            2,
            Rc::new(Node::Some(
                3,
                Rc::new(Node::Some(4, Rc::new(Node::Some(5, Rc::new(Node::None))))),
            )),
        ));
        assert_eq!(Rc::strong_count(&tail), 1);

        let head1 = Node::Some(1, Rc::clone(&tail));
        assert_eq!(Rc::strong_count(&tail), 2);

        let head2 = Node::Some(0, Rc::clone(&tail));
        assert_eq!(Rc::strong_count(&tail), 3);

        {
            let _head3 = Node::Some(-1, Rc::clone(&tail));
            assert_eq!(Rc::strong_count(&tail), 4);
        }
        assert_eq!(Rc::strong_count(&tail), 3);

        if let Node::Some(_, next) = head1 {
            if let Node::Some(x, _) = *next {
                assert_eq!(x, 2);
            } else {
                panic!("no successor of head1");
            }
        } else {
            panic!("head1 mismatch");
        }

        if let Node::Some(_, next) = head2 {
            if let Node::Some(x, _) = *next {
                assert_eq!(x, 2);
            } else {
                panic!("no successor of head2");
            }
        } else {
            panic!("head2 mismatch");
        }
    }
}
