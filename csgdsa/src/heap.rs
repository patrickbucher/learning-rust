use std::fmt::Debug;

#[derive(PartialEq)]
pub enum Order {
    Min,
    Max,
}

pub struct Heap<T: Ord + Clone + Debug> {
    tree: Vec<Element<T>>,
    order: Order,
}

#[derive(Clone, Debug)]
struct Element<T: Ord + Clone + Debug> {
    value: T,
    priority: isize,
}

impl<T: Ord + Clone + Debug> Element<T> {
    fn before(&self, other: &Self, order: &Order) -> bool {
        match order {
            Order::Min => self.priority < other.priority,
            Order::Max => self.priority > other.priority,
        }
    }
}

impl<T: Ord + Clone + Debug> Heap<T> {
    pub fn new(order: Order) -> Self {
        Heap {
            tree: Vec::new(),
            order,
        }
    }

    pub fn insert(&mut self, value: T, priority: isize) {
        self.tree.push(Element { value, priority });
        if self.tree.len() == 1 {
            return;
        }
        let mut new_index = self.tree.len() - 1;
        while new_index > 0 {
            let parent_index = (new_index - 1) / 2;
            let parent = self.tree[parent_index].clone();
            let element = self.tree[new_index].clone();
            if element.before(&parent, &self.order) {
                self.tree[parent_index] = element;
                self.tree[new_index] = parent;
                new_index = parent_index;
            } else {
                break;
            }
        }
    }

    pub fn delete(&mut self) -> Option<T> {
        if self.tree.is_empty() {
            return None;
        }
        let n = self.tree.len();
        let root = self.tree[0].clone();
        self.tree[0] = self.tree[n - 1].clone();
        let mut index = 0;
        loop {
            let trickle = self.tree[index].clone();
            let left_child_index = index * 2 + 1;
            let right_child_index = index * 2 + 2;
            let (candidate, candidate_index) = if left_child_index < n && right_child_index < n {
                let left = self.tree[left_child_index].clone();
                let right = self.tree[right_child_index].clone();
                if left.before(&right, &self.order) {
                    (left, left_child_index)
                } else {
                    (right, right_child_index)
                }
            } else if left_child_index < n {
                let left = self.tree[left_child_index].clone();
                (left, left_child_index)
            } else if right_child_index < n {
                let right = self.tree[right_child_index].clone();
                (right, right_child_index)
            } else {
                break;
            };
            if !trickle.before(&candidate, &self.order) {
                self.tree[candidate_index] = trickle.clone();
                self.tree[index] = candidate.clone();
                index = candidate_index;
            } else {
                break;
            }
        }
        self.tree.pop();
        Some(root.value)
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn holds_heap_condition(&self) -> bool {
        for (i, element) in self.tree.iter().enumerate() {
            for child_index in [i * 2 + 1, i * 2 + 2] {
                if child_index < self.tree.len() {
                    let before = match self.order {
                        Order::Min => element.priority < self.tree[child_index].priority,
                        Order::Max => element.priority > self.tree[child_index].priority,
                    };
                    if !before {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_heap() {
        let mut heap: Heap<&str> = Heap::new(Order::Max);
        heap.insert("Rowing", 9);
        heap.insert("Working", 5);
        heap.insert("Dishes", 3);
        heap.insert("Reading", 2);
        heap.insert("Sleeping", 1);
        heap.insert("Cleaning", 7);
        heap.insert("Surfing", -3);
        assert!(heap.holds_heap_condition());

        for expected in [
            Some("Rowing"),
            Some("Cleaning"),
            Some("Working"),
            Some("Dishes"),
            Some("Reading"),
            Some("Sleeping"),
            Some("Surfing"),
            None,
        ] {
            assert_eq!(heap.delete(), expected);
            assert!(heap.holds_heap_condition());
        }
    }

    #[test]
    fn test_min_heap() {
        let mut heap: Heap<&str> = Heap::new(Order::Min);
        heap.insert("Alice", 3);
        heap.insert("Bob", 5);
        heap.insert("Charlene", 2);
        heap.insert("Daniel", 1);
        heap.insert("Esmeralda", 4);
        heap.insert("Fred", 6);
        heap.insert("Gina", 0);
        assert!(heap.holds_heap_condition());

        for expected in [
            Some("Gina"),
            Some("Daniel"),
            Some("Charlene"),
            Some("Alice"),
            Some("Esmeralda"),
            Some("Bob"),
            Some("Fred"),
            None,
        ] {
            assert_eq!(heap.delete(), expected);
            assert!(heap.holds_heap_condition());
        }
    }
}
