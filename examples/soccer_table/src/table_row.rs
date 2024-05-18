use crate::parsing::MatchResult;
use std::cmp::{Ord, Ordering};

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl PartialOrd for TableRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TableRow {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_points = self.points.cmp(&other.points).reverse();
        let by_goals_diff = self.goals_diff.cmp(&other.goals_diff).reverse();
        let by_wins = self.wins.cmp(&other.wins).reverse();
        let by_name = self.team.cmp(&other.team);
        by_points.then(by_goals_diff).then(by_wins).then(by_name)
    }
}

impl TableRow {
    pub fn new(name: &str) -> TableRow {
        TableRow {
            rank: 0,
            team: String::from(name),
            points: 0,
            wins: 0,
            defeats: 0,
            ties: 0,
            goals_scored: 0,
            goals_conceded: 0,
            goals_diff: 0,
        }
    }

    pub fn from(result: MatchResult) -> (Self, Self) {
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

    pub fn combine(self, other: Self) -> Result<Self, String> {
        if self.team != other.team {
            Err(format!(
                "cannot combine rows of teams '{}' and '{}'",
                self.team, other.team
            ))
        } else {
            Ok(TableRow {
                rank: 0,
                team: self.team.clone(),
                points: self.points + other.points,
                wins: self.wins + other.wins,
                defeats: self.defeats + other.defeats,
                ties: self.ties + other.ties,
                goals_scored: self.goals_scored + other.goals_scored,
                goals_conceded: self.goals_conceded + other.goals_conceded,
                goals_diff: self.goals_diff + other.goals_diff,
            })
        }
    }
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

    #[test]
    fn combine_rows_of_different_teams() {
        let team_a_first = TableRow {
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
        let team_b_first = TableRow {
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
        let actual = team_a_first.combine(team_b_first);
        assert!(actual.is_err());
    }

    #[test]
    fn combine_rows_win_and_defeat() {
        let team_a_first = TableRow {
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
        let team_a_second = TableRow {
            rank: 0,
            team: String::from("FC Rustaceans"),
            points: 0,
            wins: 0,
            defeats: 1,
            ties: 0,
            goals_scored: 2,
            goals_conceded: 3,
            goals_diff: -1,
        };
        let expected = TableRow {
            rank: 0,
            team: String::from("FC Rustaceans"),
            points: 3,
            wins: 1,
            defeats: 1,
            ties: 0,
            goals_scored: 5,
            goals_conceded: 5,
            goals_diff: 0,
        };
        let actual = team_a_first.combine(team_a_second);
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn cmp_rows_by_points() {
        let mut a = TableRow::new("A");
        let mut b = TableRow::new("B");
        a.points = 3;
        b.points = 5;
        assert_eq!(a.cmp(&b), Ordering::Greater); // NOTE: reverse sort!
    }

    #[test]
    fn cmp_rows_by_goals_diff() {
        let mut a = TableRow::new("A");
        let mut b = TableRow::new("B");
        a.points = 5;
        b.points = 5;
        a.goals_diff = 5;
        b.goals_diff = 1;
        assert_eq!(a.cmp(&b), Ordering::Less); // NOTE: reverse sort!
    }

    #[test]
    fn cmp_rows_by_wins() {
        let mut a = TableRow::new("A");
        let mut b = TableRow::new("B");
        a.points = 5;
        b.points = 5;
        a.goals_diff = 0;
        b.goals_diff = 0;
        a.wins = 0;
        b.wins = 1;
        assert_eq!(a.cmp(&b), Ordering::Greater); // NOTE: reverse sort!
    }

    #[test]
    fn cmp_rows_by_name() {
        let mut a = TableRow::new("A");
        let mut b = TableRow::new("B");
        a.points = 5;
        b.points = 5;
        a.goals_diff = 0;
        b.goals_diff = 0;
        a.wins = 1;
        b.wins = 1;
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn sort_rows() {
        let mut a = TableRow::new("A");
        let mut b = TableRow::new("B");
        let mut c = TableRow::new("C");
        let mut d = TableRow::new("D");
        let mut e = TableRow::new("E");

        a.points = 50;
        b.points = 40;
        c.points = 40;
        d.points = 40;
        e.points = 40;

        b.goals_diff = 20;
        c.goals_diff = 10;
        d.goals_diff = 10;
        e.goals_diff = 10;

        c.wins = 8;
        d.wins = 7;
        e.wins = 7;

        let mut rows = vec![d, c, e, b, a];
        rows.sort();
        assert_eq!(rows.get(0).unwrap().team, "A");
        assert_eq!(rows.get(1).unwrap().team, "B");
        assert_eq!(rows.get(2).unwrap().team, "C");
        assert_eq!(rows.get(3).unwrap().team, "D");
        assert_eq!(rows.get(4).unwrap().team, "E");
    }
}
