pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: T) {
        let node = Node { value, next: None };
        let mut tail = &self.head;
        loop {
            if let Some(node) = tail.next {
                tail = &node.next;
            } else {
                tail.next = Some(Box::new(node));
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn append_items_sequentially() {
        let mut list: LinkedList<usize> = LinkedList::new();
    }
}
