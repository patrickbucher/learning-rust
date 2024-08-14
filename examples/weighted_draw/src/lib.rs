use rand::Rng;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Draws a random value of the keys from items_count, with a probability to draw an entry based on
/// its count (value).
pub fn draw<T: Eq + Hash + Debug + Clone>(items_count: &HashMap<T, usize>) -> Option<T> {
    let total: usize = items_count.values().sum();
    let proportions: HashMap<&T, f64> = items_count
        .iter()
        .map(|(k, v)| (k, *v as f64 / total as f64))
        .collect();
    let mut bounds: HashMap<&T, (f64, f64)> = HashMap::new();
    let mut current: f64 = 0.0;
    for (k, v) in proportions {
        bounds.insert(k, (current, current + v));
        current += v;
    }
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    for (k, (lower, upper)) in bounds {
        if lower <= x && x < upper {
            return Some(k.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_proportionally() {
        let choices = HashMap::from([("a", 90), ("b", 9), ("c", 1)]);
        let mut draws: HashMap<&str, usize> = HashMap::new();
        for _i in 0..1000 {
            let pick = draw(&choices);
            let pick = pick.unwrap();
            draws.entry(pick).and_modify(|e| *e += 1).or_insert(1);
        }
        let a = draws.get("a").unwrap();
        let b = draws.get("b").unwrap();
        let c = draws.get("c").unwrap();
        assert!(a > b);
        assert!(b > c);
    }
}
