use std::fmt::Display;

pub struct BullyingVector<T>
where
    T: PartialEq + Display,
{
    items: Vec<T>,
    bully: T,
}

impl<T> BullyingVector<T>
where
    T: PartialEq + Display,
{
    pub fn new(bully: T) -> BullyingVector<T> {
        BullyingVector {
            items: Vec::new(),
            bully,
        }
    }

    pub fn add(&mut self, item: T) -> bool {
        if item != self.bully {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.items.iter().any(|i| i == item)
    }
}

impl<T> Drop for BullyingVector<T>
where
    T: PartialEq + Display,
{
    fn drop(&mut self) {
        println!("Stop bullying {}…", self.bully);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_todds() {
        let mut no_todds = BullyingVector::new(String::from("Todd"));
        no_todds.add(String::from("Tucker"));
        no_todds.add(String::from("Eddie"));
        no_todds.add(String::from("Todd"));
        assert!(no_todds.contains(&String::from("Tucker")));
        assert!(no_todds.contains(&String::from("Eddie")));
        assert!(!no_todds.contains(&String::from("Todd")));
    }
}
