mod parsing;

use parsing::{list_relevant_files, read_lines, MatchResult};
use std::cmp::Ordering;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct TableRow {
    pub rank: u8,
    pub team: String,
    pub points: u8,
    pub wins: u8,
    pub defeats: u8,
    pub ties: u8,
    pub goals_scored: u8,
    pub goals_conceded: u8,
    pub goals_diff: i8,
}

impl TableRow {
    fn from(result: MatchResult) -> (TableRow, TableRow) {
        let mut home = TableRow {
            rank: 0,
            team: result.home_team,
            points: 0,
            wins: 0,
            defeats: 0,
            ties: 0,
            goals_scored: result.home_goals,
            goals_conceded: result.away_goals,
            goals_diff: result.home_goals as i8 - result.away_goals as i8,
        };
        let mut away = TableRow {
            rank: 0,
            team: result.away_team,
            points: 0,
            wins: 0,
            defeats: 0,
            ties: 0,
            goals_scored: result.away_goals,
            goals_conceded: result.home_goals,
            goals_diff: result.away_goals as i8 - result.home_goals as i8,
        };
        match result.home_goals.cmp(&result.away_goals) {
            Ordering::Less => {
                home.defeats = 1;
                away.wins = 1;
                away.points = 3;
            }
            Ordering::Greater => {
                home.wins = 1;
                away.defeats = 1;
                home.points = 3;
            }
            Ordering::Equal => {
                home.ties = 1;
                away.ties = 1;
                home.points = 1;
                away.points = 1;
            }
        }
        (home, away)
    }
}

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    let mut lines: Vec<String> = Vec::new();
    if let Ok(files) = list_relevant_files(dir, day) {
        for p in files {
            lines.append(&mut read_lines(&p))
        }
    }
    for line in lines {
        let r: Result<MatchResult, _> = line.try_into();
        match r {
            Ok(m) => println!("{:?}", m),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(Box::new(Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_from_home_win() {
        let result = MatchResult {
            home_team: String::from("FC Rustaceans"),
            away_team: String::from("COBOL FC 1958"),
            home_goals: 3,
            away_goals: 2,
        };
        let expected_home = TableRow {
            rank: 0,
            team: String::from("FC Rustaceans"),
            points: 3,
            wins: 1,
            defeats: 0,
            ties: 0,
            goals_scored: 3,
            goals_conceded: 2,
            goals_diff: 1,
        };
        let expected_away = TableRow {
            rank: 0,
            team: String::from("COBOL FC 1958"),
            points: 0,
            wins: 0,
            defeats: 1,
            ties: 0,
            goals_scored: 2,
            goals_conceded: 3,
            goals_diff: -1,
        };
        let (actual_home, actual_away) = TableRow::from(result);
        assert_eq!(actual_home, expected_home);
        assert_eq!(actual_away, expected_away);
    }

    #[test]
    fn rows_from_away_win() {
        let result = MatchResult {
            home_team: String::from("FC Rustaceans"),
            away_team: String::from("COBOL FC 1958"),
            home_goals: 2,
            away_goals: 4,
        };
        let expected_home = TableRow {
            rank: 0,
            team: String::from("FC Rustaceans"),
            points: 0,
            wins: 0,
            defeats: 1,
            ties: 0,
            goals_scored: 2,
            goals_conceded: 4,
            goals_diff: -2,
        };
        let expected_away = TableRow {
            rank: 0,
            team: String::from("COBOL FC 1958"),
            points: 3,
            wins: 1,
            defeats: 0,
            ties: 0,
            goals_scored: 4,
            goals_conceded: 2,
            goals_diff: 2,
        };
        let (actual_home, actual_away) = TableRow::from(result);
        assert_eq!(actual_home, expected_home);
        assert_eq!(actual_away, expected_away);
    }

    #[test]
    fn rows_from_tie() {
        let result = MatchResult {
            home_team: String::from("FC Rustaceans"),
            away_team: String::from("COBOL FC 1958"),
            home_goals: 3,
            away_goals: 3,
        };
        let expected_home = TableRow {
            rank: 0,
            team: String::from("FC Rustaceans"),
            points: 1,
            wins: 0,
            defeats: 0,
            ties: 1,
            goals_scored: 3,
            goals_conceded: 3,
            goals_diff: 0,
        };
        let expected_away = TableRow {
            rank: 0,
            team: String::from("COBOL FC 1958"),
            points: 1,
            wins: 0,
            defeats: 0,
            ties: 1,
            goals_scored: 3,
            goals_conceded: 3,
            goals_diff: 0,
        };
        let (actual_home, actual_away) = TableRow::from(result);
        assert_eq!(actual_home, expected_home);
        assert_eq!(actual_away, expected_away);
    }
}
