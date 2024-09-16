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

impl<T: Ord + Clone + Debug> Heap<T> {
    pub fn new(order: Order) -> Self {
        Heap {
            tree: Vec::new(),
            order,
        }
    }

    pub fn insert(&mut self, value: T, priority: isize) {
        self.tree.push(Element { value, priority });
        if self.tree.len() > 1 {
            let mut new_index = self.tree.len() - 1;
            while new_index > 0 {
                let parent_index = (new_index - 1) / 2;
                let parent_priority = self.tree[parent_index].priority;
                let before = match self.order {
                    Order::Min => priority < parent_priority,
                    Order::Max => priority > parent_priority,
                };
                if before {
                    let parent = self.tree[parent_index].clone();
                    let element = self.tree[new_index].clone();
                    self.tree[parent_index] = element;
                    self.tree[new_index] = parent;
                    new_index = parent_index;
                } else {
                    break;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    fn holds_heap_condition(&self) -> bool {
        for (i, element) in self.tree.iter().enumerate() {
            let i_left_child = i * 2 + 1;
            let i_right_child = i * 2 + 2;
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

    fn last(&self) -> Option<T> {
        match self.is_empty() {
            true => None,
            false => Some(self.tree[self.tree.len() - 1].value.clone()),
        }
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
    }
}
