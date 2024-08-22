pub struct Stack<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn read(&self) -> Option<T> {
        if !self.items.is_empty() {
            Some(self.items[self.items.len() - 1].clone())
        } else {
            None
        }
    }
}

impl<T: Clone> Default for Stack<T> {
    fn default() -> Self {
        Stack::new()
    }
}

pub fn reverse(s: &str) -> String {
    let mut chars = Stack::new();
    for c in s.chars() {
        chars.push(c);
    }
    let mut reversed = String::new();
    while let Some(c) = chars.pop() {
        reversed.push(c);
    }
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert_eq!(stack.size(), 0);

        stack.push(3);
        assert_eq!(stack.read(), Some(3));

        let tip = stack.pop();
        assert_eq!(tip, Some(3));
        assert_eq!(stack.pop(), None);

        let range = 0..10;
        for i in range.clone() {
            stack.push(i);
        }
        assert_eq!(stack.size(), range.len());
        for i in range.rev() {
            assert_eq!(stack.pop(), Some(i));
        }
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn reverse_string() {
        assert_eq!(reverse("abcde"), "edcba");
    }
}
