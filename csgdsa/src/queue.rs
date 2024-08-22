pub struct Queue<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn enqueue(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            let front = self.items[0].clone();
            self.items = self.items.iter().skip(1).cloned().collect();
            Some(front)
        } else {
            None
        }
    }

    pub fn read(&self) -> Option<T> {
        if !self.items.is_empty() {
            Some(self.items[self.items.len() - 1].clone())
        } else {
            None
        }
    }
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Self {
        Queue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        assert_eq!(queue.length(), 0);

        queue.enqueue(3);
        assert_eq!(queue.read(), Some(3));

        let front = queue.dequeue();
        assert_eq!(front, Some(3));

        let range = 0..10;
        for i in range.clone() {
            queue.enqueue(i);
        }
        assert_eq!(queue.length(), range.len());
        for i in range {
            assert_eq!(queue.dequeue(), Some(i));
        }
        assert_eq!(queue.length(), 0);
        assert_eq!(queue.dequeue(), None);
    }
}
