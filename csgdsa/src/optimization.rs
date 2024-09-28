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
}
