use regex::Regex;
use std::error::Error;
use std::num::ParseIntError;
use std::{fmt, fmt::Display};

fn main() {
    let results = vec![
        "FC Barcelona 2:3 Real Madrid",
        "Manchaster United 2:1 Arsenal London",
        "Bayer Leverkusen 1:0 Bayern München",
    ];
    let pattern = Regex::new("^(.+) ([0-9]+):([0-9]+) (.+)$").expect("invalid regex");
    let results = results.iter().filter_map(|r| parse(r, &pattern).ok());
    for result in results {
        println!("{}", result);
    }
}

#[derive(Debug)]
struct MatchResult {
    home_team: String,
    away_team: String,
    home_goals: usize,
    away_goals: usize,
}

impl Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}:{} {}",
            self.home_team, self.home_goals, self.away_goals, self.away_team
        )
    }
}

#[derive(Debug)]
enum ParseError {
    Mismatch,
    Number,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Mismatch => write!(f, "regex did not match"),
            ParseError::Number => write!(f, "unable to parse number"),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(_e: ParseIntError) -> Self {
        ParseError::Number
    }
}

fn parse(result: &str, pattern: &Regex) -> Result<MatchResult, ParseError> {
    let captures: Vec<&str> = pattern
        .captures_iter(result)
        .map(|c| c.extract::<4>())
        .flat_map(|(_, matches)| matches.to_vec())
        .collect();
    if captures.len() != 4 {
        return Err(ParseError::Mismatch);
    }
    let home_team = captures.get(0).ok_or(ParseError::Mismatch)?.to_string();
    let away_team = captures.get(3).ok_or(ParseError::Mismatch)?.to_string();
    let home_goals: usize = captures.get(1).ok_or(ParseError::Mismatch)?.parse()?;
    let away_goals: usize = captures.get(2).ok_or(ParseError::Mismatch)?.parse()?;
    Ok(MatchResult {
        home_team,
        away_team,
        home_goals,
        away_goals,
    })
}
