pub struct SetArray<T: Eq + Clone> {
    items: Vec<T>,
}

impl<T> Default for SetArray<T>
where
    T: Eq + Clone,
{
    fn default() -> Self {
        SetArray { items: Vec::new() }
    }
}

impl<T> SetArray<T>
where
    T: Eq + Clone,
{
    pub fn insert(&mut self, value: T) {
        if !self.contains(&value) {
            self.items.push(value);
        }
    }

    pub fn get_values(&self) -> Vec<T> {
        self.items.clone()
    }

    fn contains(&self, value: &T) -> bool {
        for v in &self.items {
            if *v == *value {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_unique() {
        let mut numbers: SetArray<usize> = Default::default();
        numbers.insert(1);
        numbers.insert(2);
        numbers.insert(3);
        let expected = vec![1, 2, 3];
        let actual = numbers.get_values();
        assert_eq!(expected, actual);
    }

    #[test]
    fn insert_redundant() {
        let mut numbers: SetArray<usize> = Default::default();
        numbers.insert(1);
        numbers.insert(2);
        numbers.insert(3);
        numbers.insert(2); // redundant
        let expected = vec![1, 2, 3];
        let actual = numbers.get_values();
        assert_eq!(expected, actual);
    }
}
