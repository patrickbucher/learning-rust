use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct MatchResult {
    pub home_team: String,
    pub away_team: String,
    pub home_goals: u8,
    pub away_goals: u8,
}

impl MatchResult {
    pub fn parse_all(lines: Vec<String>) -> Result<Vec<Result<MatchResult, String>>, String> {
        let pattern = "^(.+) ([0-9]+):([0-9]+) (.+)$";
        match Regex::new(pattern) {
            Ok(p) => Ok(lines.iter().map(|l| Self::parse(l.into(), &p)).collect()),
            Err(err) => Err(format!("compile regex '{}': {}", pattern, err)),
        }
    }

    fn parse(line: String, pattern: &Regex) -> Result<MatchResult, String> {
        let err = format!("'{line}' does not match '{pattern}'");
        let caps = pattern.captures(&line).ok_or(err.clone())?;
        let home_team = caps.get(1).ok_or("missing home_team")?.as_str();
        let home_goals = caps.get(2).ok_or("missing home_goals")?.as_str();
        let away_goals = caps.get(3).ok_or("missing away_goals")?.as_str();
        let away_team = caps.get(4).ok_or("missing away_team")?.as_str();
        match (home_goals.parse::<u8>(), away_goals.parse::<u8>()) {
            (Ok(home_goals), Ok(away_goals)) => Ok(MatchResult {
                home_team: home_team.into(),
                away_team: away_team.into(),
                home_goals,
                away_goals,
            }),
            _ => Err(err),
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
