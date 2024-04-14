pub mod tree {
    pub enum Tree<T> {
        Node(Box<Tree<T>>, T, Box<Tree<T>>),
        Leaf(T),
    }

    pub fn build_tree() -> Tree<i32> {
        Tree::Node::<i32>(
            Box::new(Tree::Leaf::<i32>(1)),
            2,
            Box::new(Tree::Leaf::<i32>(3)),
        )
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
    }
}
