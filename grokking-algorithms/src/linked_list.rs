struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn prepend(self, value: T) -> Self {
        LinkedList {
            head: Some(Box::new(Node {
                value,
                next: self.head,
            })),
        }
    }

    pub fn get_values(&self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();
        let mut node: &Option<Box<Node<T>>> = &self.head;
        while let Some(ref inner) = node {
            values.push(inner.value.clone());
            node = &inner.next;
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_values_of_empty_list() {
        let list: LinkedList<u8> = LinkedList::new();
        let expected = Vec::new();
        let actual = list.get_values();
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_values_of_singleton_list() {
        let list: LinkedList<u8> = LinkedList::new();
        let list = list.prepend(42);
        let expected = vec![42];
        let actual = list.get_values();
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_values_of_non_empty_list() {
        let mut list = LinkedList::new();
        let expected: Vec<u8> = vec![4, 3, 2, 1, 0];
        for i in 0..5 {
            list = list.prepend(i);
        }
        let actual = list.get_values();
        assert_eq!(actual, expected);
    }
}
