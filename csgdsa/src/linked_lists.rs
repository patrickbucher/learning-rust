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
        let new = Node { value, next: None };
        let head = match &self.head {
            Some(node) => node,
            None => {
                self.head = Some(Box::new(new));
                return;
            }
        };
        let mut temp: &mut Box<Node<T>> = head;
        while let Some(ref mut next) = &temp.next {
            temp = &mut next;
        }
        temp.next = Some(Box::new(new));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn append_items_sequentially() {
        let mut list: LinkedList<usize> = LinkedList::new();
    }
}
