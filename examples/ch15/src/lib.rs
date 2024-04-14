use std::ops::{Deref, DerefMut};

enum Tree<T> {
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
    Leaf(T),
}

fn build_tree() -> Tree<i32> {
    Tree::Node::<i32>(
        Box::new(Tree::Leaf::<i32>(1)),
        2,
        Box::new(Tree::Leaf::<i32>(3)),
    )
}

struct VerboseBox<T> {
    value: T,
}

impl<T> VerboseBox<T> {
    fn new(value: T) -> VerboseBox<T> {
        VerboseBox { value }
    }
}

impl<T> Deref for VerboseBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("dereferencing");
        &self.value
    }
}

struct CounterBox<T> {
    value: T,
    writes: usize,
}

impl<T> CounterBox<T> {
    fn new(value: T) -> CounterBox<T> {
        CounterBox { value, writes: 0 }
    }

    fn write_count(&self) -> usize {
        self.writes
    }
}

impl<T> Deref for CounterBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for CounterBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.writes += 1;
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_tree() {
        let tree = build_tree();
        if let Tree::Node(left, value, right) = tree {
            assert_eq!(value, 2);
            if let Tree::Leaf(left_value) = *left {
                assert_eq!(left_value, 1);
            } else {
                panic!("left tree is not a leaf");
            }
            if let Tree::Leaf(right_value) = *right {
                assert_eq!(right_value, 3);
            } else {
                panic!("right tree is not a leaf");
            }
        } else {
            panic!("tree is not a node");
        }
    }

    #[test]
    fn follow_reference() {
        let a = 4;
        let b = &a;
        let c = Box::new(a);

        assert_eq!(a, 4);
        assert_eq!(*b, 4);
        assert_eq!(*c, 4);
    }

    #[test]
    fn deref_verbose_box() {
        let a = 3;
        let b = &a;
        let c = Box::new(a);
        let d = VerboseBox::new(a);

        assert_eq!(a, 3);
        assert_eq!(*b, 3);
        assert_eq!(*c, 3);
        assert_eq!(*d, 3);
    }

    #[test]
    fn deref_counter_box() {
        let mut a = CounterBox::new(13);
        *a += 1;
        *a += 1;
        assert_eq!(a.write_count(), 2);
    }
}
