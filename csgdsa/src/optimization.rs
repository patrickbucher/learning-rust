use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};

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

pub fn find_best_transaction(prices: &[usize]) -> (usize, usize) {
    let mut best_buy_day: usize = 0;
    let mut potentially_best_buy_day: usize = 0;
    let mut potentially_min_buy_price = usize::MAX;
    let mut best_sell_day: usize = 0;
    let mut max_sell_price = usize::MIN;
    for (day, price) in prices.iter().enumerate() {
        if *price > max_sell_price {
            max_sell_price = *price;
            best_sell_day = day;
            best_buy_day = potentially_best_buy_day;
        }
        if *price < potentially_min_buy_price {
            potentially_min_buy_price = *price;
            potentially_best_buy_day = day;
        }
    }
    (best_buy_day, best_sell_day)
}

pub fn find_highest_product(numbers: &[isize]) -> usize {
    let mut min1 = isize::MAX;
    let mut min2 = isize::MAX;
    let mut max1 = isize::MIN;
    let mut max2 = isize::MIN;
    for number in numbers {
        if *number < min1 {
            min2 = min1;
            min1 = *number;
        } else if *number < min2 {
            min2 = *number;
        }
        if *number > max1 {
            max2 = max1;
            max1 = *number;
        } else if *number > max2 {
            max2 = *number;
        }
    }
    let p_min = min1 * min2;
    let p_max = max1 * max2;
    if p_min > p_max {
        p_min as usize
    } else {
        p_max as usize
    }
}

pub fn sort_bound(lower: f32, upper: f32, step: f32, values: &[f32]) -> Vec<f32> {
    let mut sorted: Vec<f32> = Vec::new();
    let factor = 1.0 / step;
    let mut counts: HashMap<isize, usize> = HashMap::new();
    for value in values {
        counts
            .entry((value * factor) as isize)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    let range: Vec<isize> = (((lower * factor) as isize)..=((upper * factor) as isize)).collect();
    let mut range = range.iter();
    let mut candidate = match range.next() {
        Some(number) => number,
        None => return sorted,
    };
    loop {
        let next = match counts.entry(*candidate) {
            Entry::Occupied(mut e) => {
                if *e.get() > 0 {
                    sorted.push(*candidate as f32 / factor);
                    *e.get_mut() -= 1;
                }
                *e.get() == 0
            }
            Entry::Vacant(_) => true,
        };
        if next {
            candidate = match range.next() {
                Some(number) => number,
                None => break,
            };
        }
    }
    sorted
}

pub fn find_longest_consecutive_sequence(numbers: &[usize]) -> Vec<usize> {
    if numbers.is_empty() {
        return Vec::new();
    }
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    let set: HashSet<usize> = numbers.iter().copied().collect();
    let mut longest: Vec<usize> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    for number in min..=max {
        if set.contains(&number) {
            current.push(number);
        } else if current.len() > longest.len() {
            longest = current.clone();
            current = Vec::new();
        } else {
            current = Vec::new();
        }
    }
    longest
}

#[cfg(test)]
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
    fn test_find_best_transaction() {
        let (buy, sell) = find_best_transaction(&[10, 7, 5, 8, 11, 2, 6]);
        assert_eq!((buy, sell), (2, 4));
    }

    #[test]
    fn test_find_highest_product() {
        assert_eq!(find_highest_product(&[5, -10, -6, 9, 4]), 60);
        assert_eq!(find_highest_product(&[5, 10, -6, 9, 4]), 90);
    }

    #[test]
    fn test_sort_bound() {
        let readings: Vec<f32> = vec![98.6, 98.0, 97.1, 99.0, 98.9, 97.8, 98.5, 98.2, 98.0, 97.1];
        let actual: Vec<f32> = sort_bound(97.0, 99.0, 0.1, &readings);
        let expected: Vec<f32> = vec![97.1, 97.1, 97.8, 98.0, 98.0, 98.2, 98.5, 98.6, 98.9, 99.0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_longest_consecutive_sequence() {
        assert_eq!(
            find_longest_consecutive_sequence(&[10, 5, 12, 3, 55, 30, 4, 11, 2]),
            vec![2, 3, 4, 5]
        );
        assert_eq!(
            find_longest_consecutive_sequence(&[19, 13, 15, 12, 18, 14, 17, 11]),
            vec![11, 12, 13, 14, 15]
        );
    }
}
