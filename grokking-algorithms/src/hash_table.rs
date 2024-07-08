pub struct HashTable {
    slots: Vec<Vec<(String, String)>>,
}

impl HashTable {
    pub fn new(n_slots: usize) -> HashTable {
        if n_slots < 1 {
            panic!("HashTable must have at least one slot.");
        }
        let mut slots: Vec<Vec<(String, String)>> = Vec::new();
        for _ in 0..n_slots {
            slots.push(Vec::new());
        }
        HashTable { slots }
    }

    pub fn insert(&mut self, key: String, value: String) {
        let i = hash(&key, self.slots.len());
        self.slots[i].push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let i = hash(key, self.slots.len());
        if i > self.slots.len() {
            return None;
        }
        for (k, v) in &self.slots[i] {
            if k == key {
                return Some(v);
            }
        }
        return None;
    }

    pub fn contains(&self, key: &str) -> bool {
        let i = hash(key, self.slots.len());
        if i > self.slots.len() {
            return false;
        }
        for (k, _) in &self.slots[i] {
            if k == key {
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
        table.insert("foo".into(), "bar".into());
        assert!(table.contains("foo"));
    }

    #[test]
    fn retrieve_value() {
        let mut table = HashTable::new(3);
        table.insert("a".into(), "Alice".into());
        table.insert("b".into(), "Bob".into());
        table.insert("c".into(), "Charlene".into());
        table.insert("d".into(), "Dan".into());
        table.insert("e".into(), "Eric".into());
        assert_eq!(table.get("a"), Some("Alice"));
        assert_eq!(table.get("b"), Some("Bob"));
        assert_eq!(table.get("c"), Some("Charlene"));
        assert_eq!(table.get("d"), Some("Dan"));
        assert_eq!(table.get("e"), Some("Eric"));
        assert_eq!(table.get("f"), None);
    }

    #[test]
    fn hash_singleton_string() {
        let value = "a"; // 97
        let expected = 97;
        let actual = hash(value, 100);
        assert_eq!(actual, expected);
        let expected = 7;
        let actual = hash(value, 10);
        assert_eq!(actual, expected);
    }
}
