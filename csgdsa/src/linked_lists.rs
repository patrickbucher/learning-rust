pub struct Node<T: Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    fn new(value: T) -> Self {
        Node { value, next: None }
    }

    fn prepend(self, value: T) -> Self {
        Node {
            value,
            next: Some(Box::new(self)),
        }
    }

    fn get_values(&self) -> Vec<T> {
        let mut values: Vec<T> = vec![self.value.clone()];
        let mut temp = &self.next;
        while let Some(node) = temp {
            values.push(node.value.clone());
            temp = &node.next;
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn append_items_sequentially() {
        let mut head: Node<usize> = Node::new(4);
        let head = head.prepend(3);
        let head = head.prepend(2);
        let head = head.prepend(1);
        assert_eq!(head.get_values(), vec![1, 2, 3, 4]);
    }
}
