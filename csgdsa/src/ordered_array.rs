use std::cmp::Ordering;
use std::fmt::Debug;

pub struct OrderedArray<T: Ord + Clone + Debug> {
    array: Vec<T>,
}

impl<T> OrderedArray<T>
where
    T: Ord + Clone + Debug,
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

    pub fn search_linear(&self, needle: T) -> Option<usize> {
        for (i, v) in self.array.iter().enumerate() {
            if *v == needle {
                return Some(i);
            } else if *v > needle {
                return None;
            }
        }
        None
    }

    pub fn search_binary(&self, needle: T) -> Option<usize> {
        let mut low = 0;
        let mut high = self.array.len() - 1;
        while low <= high {
            let mid = (low + high) / 2;
            let val = self.array[mid].clone();
            match needle.cmp(&val) {
                Ordering::Equal => return Some(mid),
                Ordering::Greater => low = mid + 1,
                Ordering::Less => high = mid - 1,
            }
        }
        None
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

    #[test]
    fn search_linear_success() {
        let mut numbers = OrderedArray::new();
        for i in 0..10 {
            numbers.insert(i * 10);
        }
        for i in 0..10 {
            let expected = Some(i);
            let actual = numbers.search_linear(i * 10);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn search_linear_fail() {
        let mut numbers = OrderedArray::new();
        for i in 0..10 {
            numbers.insert(i * 10);
        }
        for i in 0..10 {
            let expected = None;
            let actual = numbers.search_linear(i * 10 + 1);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn search_binary_success() {
        let mut numbers = OrderedArray::new();
        for i in 0..10 {
            numbers.insert(2_usize.pow(i));
        }
        for i in 0..10 {
            let expected = Some(i as usize);
            let actual = numbers.search_binary(2_usize.pow(i));
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn search_binary_fail() {
        let mut numbers = OrderedArray::new();
        for i in 0..10 {
            numbers.insert(2_usize.pow(i));
        }
        for i in 0..10 {
            let expected = None;
            let actual = numbers.search_binary(2_usize.pow(i) + 1000);
            assert_eq!(expected, actual);
        }
    }
}
