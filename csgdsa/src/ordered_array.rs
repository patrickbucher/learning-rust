pub struct OrderedArray<T: Ord + Clone> {
    array: Vec<T>,
}

impl<T> OrderedArray<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        OrderedArray { array: Vec::new() }
    }

    pub fn insert(&mut self, item: T) {
        let mut pos = 0;
        for v in &self.array {
            if item > *v {
                pos += 1;
            } else {
                break;
            }
        }
        self.array.push(item);
        for i in ((pos + 1)..self.array.len()).rev() {
            let tmp = self.array[i - 1].clone();
            self.array[i - 1] = self.array[i].clone();
            self.array[i] = tmp;
        }
    }

    pub fn get_values(&self) -> Vec<T> {
        self.array.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_in_order_small() {
        let mut numbers = OrderedArray::new();
        numbers.insert(7);
        numbers.insert(3);
        numbers.insert(8);
        numbers.insert(4);
        numbers.insert(1);
        numbers.insert(9);
        numbers.insert(0);
        numbers.insert(6);
        numbers.insert(2);
        numbers.insert(5);
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let actual = numbers.get_values();
        assert_eq!(actual, expected);
    }

    #[test]
    fn insert_in_order_big() {
        let mut numbers = OrderedArray::new();
        for i in (-100..100).rev() {
            numbers.insert(i);
        }
        let expected: Vec<isize> = (-100..100).collect();
        let actual = numbers.get_values();
        assert_eq!(actual, expected);
    }
}
