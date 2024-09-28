use std::collections::HashSet;

pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub team: String,
}

impl Player {
    pub fn new(first_name: &str, last_name: &str, team: &str) -> Self {
        Player {
            first_name: first_name.into(),
            last_name: last_name.into(),
            team: team.into(),
        }
    }
}

pub fn play_in_both_sports(players_a: &Vec<Player>, players_b: &Vec<Player>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let mut full_names: HashSet<String> = HashSet::new();
    for player in players_a {
        let full_name = format!("{} {}", player.first_name, player.last_name);
        full_names.insert(full_name);
    }
    for player in players_b {
        let full_name = format!("{} {}", player.first_name, player.last_name);
        if full_names.contains(&full_name) {
            results.push(full_name);
        }
    }
    results
}

pub fn find_missing_integer(numbers: &[usize]) -> Option<usize> {
    if numbers.len() < 2 {
        return None;
    }
    let mut min: Option<usize> = None;
    let mut max: Option<usize> = None;
    let mut found: HashSet<usize> = HashSet::new();
    for number in numbers {
        min = match min {
            Some(old) => {
                if *number < old {
                    Some(*number)
                } else {
                    Some(old)
                }
            }
            None => Some(*number),
        };
        max = match max {
            Some(old) => {
                if *number > old {
                    Some(*number)
                } else {
                    Some(old)
                }
            }
            None => Some(*number),
        };
        found.insert(*number);
    }
    match (min, max) {
        (Some(a), Some(b)) => {
            let expected: HashSet<usize> = (a..=b).collect();
            let missing: HashSet<&usize> = expected.difference(&found).collect();
            if missing.len() != 1 {
                None
            } else {
                missing.iter().next().copied().copied()
            }
        }
        _ => None,
    }
}

pub fn sort_bound(lower: f32, upper: f32, step: f32, values: &[f32]) -> Vec<f32> {
    let mut sorted: Vec<f32> = Vec::new();
    let factor = 1.0 / step;

    // TODO: implement as a counting HsshMap
    let actual: Vec<isize> = values.into_iter().map(|f| (f * factor) as isize).collect();

    let range: Vec<isize> = (((lower * factor) as isize)..((upper * factor) as isize)).collect();
    for x in range {
        if actual.contains(&x) {
            sorted.push(x as f32 / factor);
        }
    }
    sorted
}

mod tests {
    use super::*;

    #[test]
    fn test_play_in_both_sports() {
        let basketball_players: Vec<Player> = vec![
            Player::new("Jill", "Huang", "Gators"),
            Player::new("Janko", "Barton", "Sharks"),
            Player::new("Wanda", "Vakulskas", "Sharks"),
            Player::new("Jill", "Moloney", "Gators"),
            Player::new("Luuk", "Watkins", "Gators"),
        ];
        let football_players: Vec<Player> = vec![
            Player::new("Hanzla", "Radosti", "32ers"),
            Player::new("Tina", "Watkins", "Barleycorns"),
            Player::new("Alex", "Patel", "32ers"),
            Player::new("Jill", "Huang", "Barleycorns"),
            Player::new("Wanda", "Vakulskas", "Barleycorns"),
        ];
        let expected = vec![String::from("Jill Huang"), String::from("Wanda Vakulskas")];
        let actual = play_in_both_sports(&basketball_players, &football_players);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_missing_integer() {
        assert_eq!(find_missing_integer(&[2, 3, 0, 6, 1, 5]), Some(4));
        assert_eq!(find_missing_integer(&[8, 2, 3, 9, 4, 7, 5, 0, 6]), Some(1));
    }

    #[test]
    fn test_sort_bound() {
        let readings: Vec<f32> = vec![98.6, 98.0, 97.1, 99.0, 98.9, 97.8, 98.5, 98.2, 98.0, 97.1];
        let actual: Vec<f32> = sort_bound(97.0, 99.0, 0.1, &readings);
        let expected: Vec<f32> = vec![97.1, 97.1, 97.8, 98.0, 98.0, 98.2, 98.5, 98.6, 98.9, 99.0];
        assert_eq!(actual, expected);
    }
}
