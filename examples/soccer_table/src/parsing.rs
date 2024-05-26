use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct MatchResult {
    pub home_team: String,
    pub away_team: String,
    pub home_goals: u8,
    pub away_goals: u8,
}

#[derive(Clone, Debug)]
pub enum ParseError {
    RegexSyntax { pat: String, err: regex::Error },
    RegexMismatch { pat: String, val: String },
    NumberParsing { val: String, err: ParseIntError },
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParseError::RegexSyntax { pat: p, err: e } => write!(f, "invalid regex {p}: {e}"),
            ParseError::RegexMismatch { pat: p, val: s } => write!(f, "{s} did not match {p}"),
            ParseError::NumberParsing { val: v, err: e } => write!(f, "parse {v}: {e}"),
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
            Err(e) => Err(ParseError::RegexSyntax {
                pat: pattern.into(),
                err: e,
            }),
        }
    }

    fn parse(line: String, pattern: &Regex) -> Result<MatchResult, ParseError> {
        let [ht, hg, ag, at] = pattern
            .captures_iter(&line)
            .map(|c| c.extract::<4>())
            .flat_map(|(_, matches)| matches)
            .collect::<Vec<&str>>()
            .try_into()
            .or(Err(ParseError::RegexMismatch {
                pat: pattern.as_str().into(),
                val: line.clone(),
            }))?;
        let home_team = ht.to_string();
        let away_team = at.to_string();
        let home_goals = hg.parse::<u8>().map_err(|e| ParseError::NumberParsing {
            val: hg.into(),
            err: e,
        })?;
        let away_goals = ag.parse::<u8>().map_err(|e| ParseError::NumberParsing {
            val: ag.into(),
            err: e,
        })?;
        Ok(MatchResult {
            home_team,
            away_team,
            home_goals,
            away_goals,
        })
    }
}

pub fn list_relevant_files(dir: &Path, day: Option<usize>) -> Result<Vec<PathBuf>, ()> {
    let pattern = Regex::new("([0-9]+).txt$").or(Err(()))?;
    match list_files(dir) {
        Ok(files) => {
            // TODO: solution with zip?
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
    use std::env;
    use std::fs;

    #[test]
    fn extract_days_from_files() {
        let relevant_day = 50;
        let total_days = 99;

        let mut test_dir = PathBuf::from(env::temp_dir());
        test_dir.push("soccer-table-test");
        let path = test_dir.as_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }

        fs::create_dir(path).unwrap();
        let mut expected: Vec<PathBuf> = Vec::new();
        for i in 1..=total_days {
            let name = format!("{i:02}.txt");
            let mut path_buf = PathBuf::from(path);
            path_buf.push(name);
            fs::File::create_new(path_buf.as_path()).unwrap();
            if i <= relevant_day {
                expected.push(path_buf);
            }
        }

        let mut actual = list_relevant_files(path, Some(relevant_day)).unwrap();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn must_parse_result() {
        let raw = String::from("The Rustaceans 3:2 COBOL FC 1958");
        let expected = MatchResult {
            home_team: String::from("The Rustaceans"),
            away_team: String::from("COBOL FC 1958"),
            home_goals: 3,
            away_goals: 2,
        };
        let parsed = MatchResult::parse_all(vec![raw]).unwrap();
        let result = parsed.get(0).unwrap();
        let actual = result.as_ref().unwrap();
        assert_eq!(*actual, expected);
    }
}
