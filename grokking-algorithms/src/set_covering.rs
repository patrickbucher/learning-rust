use std::collections::{HashMap, HashSet};

pub fn cover(
    required: &HashSet<&str>,
    options: &HashMap<&str, HashSet<&str>>,
) -> Result<HashSet<String>, String> {
    let mut solution: HashSet<String> = HashSet::new();
    let mut required: HashSet<String> = required.iter().map(|s| s.to_string()).collect();
    while !required.is_empty() {
        let mut best_option: Option<String> = None;
        let mut best_coverage: HashSet<String> = HashSet::new();
        for (option, new_coverage) in options {
            let new_coverage: HashSet<String> =
                new_coverage.iter().map(|s| s.to_string()).collect();
            let potential: HashSet<String> =
                required.intersection(&new_coverage).cloned().collect();
            if potential.len() > best_coverage.len() {
                best_option = Some(option.to_string());
                best_coverage = potential.clone();
                required = required.difference(&potential).cloned().collect();
            }
        }
        match best_option {
            Some(option) => solution.insert(option),
            None => return Err(format!("no best option for {required:?}")),
        };
    }
    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover_states_with_radio_stations_all_covered() {
        let states: HashSet<&str> = HashSet::from(["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]);
        let stations: HashMap<&str, HashSet<&str>> = HashMap::from([
            ("kone", HashSet::from(["id", "nv", "ut"])),
            ("ktwo", HashSet::from(["wa", "id", "mt"])),
            ("kthree", HashSet::from(["or", "nv", "ca"])),
            ("kfour", HashSet::from(["nv", "ut"])),
            ("kfive", HashSet::from(["ca", "az"])),
        ]);
        let coverage = cover(&states, &stations).unwrap();
        let covered: HashSet<&str> = coverage
            .into_iter()
            .map(|s| stations.get(&s as &str).unwrap())
            .flatten()
            .map(|s| *s)
            .collect();
        assert!(covered
            .difference(&states)
            .map(|s| *s)
            .collect::<HashSet<&str>>()
            .is_empty());
    }
}
