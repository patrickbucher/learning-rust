pub struct Node<T: Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }

    pub fn prepend(self, value: T) -> Self {
        Node {
            value,
            next: Some(Box::new(self)),
        }
    }

    pub fn append(&mut self, value: T) {
        let next = Some(Box::new(Node { value, next: None }));
        if self.next.is_none() {
            self.next = next;
            return;
        }
        let mut temp = &mut self.next;
        while let Some(node) = temp {
            if node.next.is_some() {
                temp = &mut node.next;
            } else {
                node.next = next;
                return;
            }
        }
    }

    pub fn get_values(&self) -> Vec<T> {
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

    #[test]
    fn test_prepend() {
        let head: Node<usize> = Node::new(4);
        let head = head.prepend(3);
        let head = head.prepend(2);
        let head = head.prepend(1);
        assert_eq!(head.get_values(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_append() {
        let mut head: Node<usize> = Node::new(1);
        head.append(2);
        head.append(3);
        head.append(4);
        assert_eq!(head.get_values(), vec![1, 2, 3, 4]);
    }
}
