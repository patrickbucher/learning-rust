use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct MatchResult {
    pub home_team: String,
    pub away_team: String,
    pub home_goals: u8,
    pub away_goals: u8,
}

#[derive(Debug)]
pub enum ParseError {
    RegexMismatch,
    NumberParsing,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParseError::RegexMismatch => write!(f, "regex mismatch"),
            ParseError::NumberParsing => write!(f, "number parsing"),
        }
    }
}

impl MatchResult {
    pub fn parse_all(
        lines: Vec<String>,
    ) -> Result<Vec<Result<MatchResult, ParseError>>, ParseError> {
        let pattern = "^(.+) ([0-9]+):([0-9]+) (.+)$";
        match Regex::new(pattern) {
            Ok(p) => Ok(lines.iter().map(|l| Self::parse(l.into(), &p)).collect()),
            Err(_) => Err(ParseError::RegexMismatch),
        }
    }

    fn parse(line: String, pattern: &Regex) -> Result<MatchResult, ParseError> {
        let [ht, hg, ag, at] = pattern
            .captures_iter(&line)
            .map(|c| c.extract::<4>())
            .flat_map(|(_, matches)| matches)
            .collect::<Vec<&str>>()
            .try_into()
            .or(Err(ParseError::RegexMismatch))?;
        match (hg.parse::<u8>(), ag.parse::<u8>()) {
            (Ok(home_goals), Ok(away_goals)) => Ok(MatchResult {
                home_team: ht.to_string(),
                away_team: at.to_string(),
                home_goals,
                away_goals,
            }),
            _ => Err(ParseError::NumberParsing),
        }
    }
}

pub fn list_relevant_files(dir: &Path, day: Option<usize>) -> Result<Vec<PathBuf>, ()> {
    let pattern = Regex::new("([0-9]+).txt$").or(Err(()))?;
    match list_files(dir) {
        Ok(files) => {
            let files_to_days: HashMap<_, _> = files
                .iter()
                .map(|f| (f, extract_day(f, &pattern)))
                .map(|(p, o)| (p, o.unwrap_or(0)))
                .filter(|(_, d)| *d != 0)
                .filter(|(_, d)| day.is_none() || *d <= day.unwrap_or(usize::MAX))
                .collect();
            Ok(files_to_days.keys().map(PathBuf::from).collect())
        }
        Err(()) => Err(()),
    }
}

pub fn read_lines(file: &Path) -> Vec<String> {
    match fs::read_to_string(file) {
        Ok(s) => s.trim().split('\n').map(|s| s.trim().to_string()).collect(),
        Err(_) => Vec::new(),
    }
}

fn list_files(dir: &Path) -> Result<Vec<PathBuf>, ()> {
    if !dir.exists() || !dir.is_dir() {
        return Err(());
    }
    let mut files: Vec<_> = Vec::new();
    let entries = dir.read_dir().or(Err(()))?;
    for entry in entries.flatten() {
        if let Ok(t) = entry.file_type() {
            if t.is_file() {
                files.push(dir.join(entry.path()));
            }
        }
    }
    Ok(files)
}

fn extract_day(file: &Path, pattern: &Regex) -> Option<usize> {
    let file_name = file.to_str().unwrap_or("");
    pattern
        .captures(file_name)
        .and_then(|c| c.get(1).map(|m| m.as_str()))
        .and_then(|m| m.parse::<usize>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_parse_result() {
        let raw = String::from("The Rustaceans 3:2 COBOL FC 1958");
        let expected = MatchResult {
            home_team: String::from("The Rustaceans"),
            away_team: String::from("COBOL FC 1958"),
            home_goals: 3,
            away_goals: 2,
        };
        let actual: MatchResult = raw.try_into().unwrap();
        assert_eq!(actual, expected);
    }
}
