use std::collections::{HashMap, HashSet};

pub fn cover(
    required: &HashSet<String>,
    options: &HashMap<String, HashSet<String>>,
) -> Result<HashSet<String>, String> {
    let mut solution: HashSet<String> = HashSet::new();
    let mut required: HashSet<String> = required.clone();
    while !required.is_empty() {
        let mut best_option: Option<String> = None;
        let mut best_coverage: HashSet<String> = HashSet::new();
        for (option, new_coverage) in options {
            let potential: HashSet<String> = required.intersection(new_coverage).cloned().collect();
            if potential.len() > best_coverage.len() {
                best_option = Some(option.clone());
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
        let states: HashSet<String> =
            HashSet::from(["mt", "wa", "or", "id", "nv", "ut", "ca", "az"].map(|s| s.to_string()));
        let stations: HashMap<String, HashSet<String>> = HashMap::from([
            (
                "kone".into(),
                HashSet::from(["id", "nv", "ut"].map(|s| s.to_string())),
            ),
            (
                "ktwo".into(),
                HashSet::from(["wa", "id", "mt"].map(|s| s.to_string())),
            ),
            (
                "kthree".into(),
                HashSet::from(["or", "nv", "ca"].map(|s| s.to_string())),
            ),
            (
                "kfour".into(),
                HashSet::from(["nv", "ut"].map(|s| s.to_string())),
            ),
            (
                "kfive".into(),
                HashSet::from(["ca", "az"].map(|s| s.to_string())),
            ),
        ]);
        let coverage = cover(&states, &stations).unwrap();
        let covered: HashSet<String> = coverage
            .into_iter()
            .map(|s| stations.get(&s).unwrap())
            .flatten()
            .cloned()
            .collect();
        assert!(covered
            .difference(&states)
            .collect::<HashSet<&String>>()
            .is_empty());
    }
}
