use std::collections::{HashMap, HashSet};

pub fn cover(things: &HashSet<String>, options: &HashMap<String, String>) -> HashSet<String> {
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
