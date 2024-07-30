use std::collections::{HashMap, HashSet};

pub fn cover(
    required: &HashSet<String>,
    options: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    let mut solution: HashSet<String> = HashSet::new();
    let mut required_covered: HashSet<String> = HashSet::new();
    let mut best_option: Option<String> = None;
    for (option, option_coverage) in options {
        let covered: HashSet<String> = required.intersection(option_coverage).cloned().collect();
        if covered.len() > required_covered.len() {
            best_option = Some(option.clone());
            required_covered = covered;
        }
    }
    match best_option {
        Some(option) => solution.insert(option),
        None => true, // TODO
    };
    HashSet::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover_states_with_radio_stations() {
        let states = HashSet::from(["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]);
        let stations = HashMap::from([
            ("kone", HashSet::from(["id", "nv", "ut"])),
            ("ktwo", HashSet::from(["wa", "id", "mt"])),
            ("kthree", HashSet::from(["or", "nv", "ca"])),
            ("kfour", HashSet::from(["nv", "ut"])),
            ("kfive", HashSet::from(["ca", "az"])),
        ]);
        println!("{states:?} {stations:?}");
    }
}
