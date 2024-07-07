pub struct HashTable {
    slots: Vec<Vec<String>>,
}

impl HashTable {
    pub fn new(n_slots: usize) -> HashTable {
        if n_slots < 1 {
            panic!("HashTable must have at least one slot.");
        }
        let mut slots: Vec<Vec<String>> = Vec::new();
        for _ in 0..n_slots {
            slots.push(Vec::new());
        }
        HashTable { slots }
    }

    pub fn insert(&mut self, value: String) {
        let i = hash(&value, self.slots.len());
        self.slots[i].push(value);
    }

    pub fn contains(&self, value: &str) -> bool {
        let i = hash(value, self.slots.len());
        if i > self.slots.len() {
            return false;
        }
        for e in &self.slots[i] {
            if e == value {
                return true;
            }
        }
        false
    }
}

fn hash(value: &str, max: usize) -> usize {
    let mut sum: usize = 0;
    for c in value.chars() {
        sum += c as usize;
    }
    sum % max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_contains_value() {
        let table = HashTable::new(10);
        assert!(!table.contains("foobar"));
    }

    #[test]
    fn contains_value() {
        let mut table = HashTable::new(10);
        table.insert("foobar".into());
        assert!(table.contains("foobar"));
    }
}
