use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    let mut lines: Vec<String> = Vec::new();
    println!("dir: {:?}, day: {}", dir, day.map_or(0, |v| v));
    if let Ok(files) = list_relevant_files(dir, day) {
        for p in files {
            lines.append(&mut read_lines(&p))
        }
    }
    for line in lines {
        println!("{}", line);
        let r: Result<MatchResult, _> = line.try_into();
        match r {
            Ok(m) => println!("{:?}", m),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(Box::new(Vec::new()))
}

#[derive(Debug)]
struct MatchResult {
    home_team: String,
    away_team: String,
    home_goals: u8,
    away_goals: u8,
}

impl TryFrom<String> for MatchResult {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let pattern = "^(.+) ([0-9]+):([0-9]+) (.+)$";
        match Regex::new(pattern) {
            Ok(p) => match p.captures(&value) {
                Some(groups) => {
                    let matches: Vec<&str> = groups
                        .iter()
                        .filter(|g| g.is_some())
                        .map(|g| g.unwrap().as_str())
                        .collect();
                    for m in matches {
                        println!("match: {m}");
                    }
                    return Ok(MatchResult {
                        home_team: "".into(),
                        away_team: "".into(),
                        home_goals: 0,
                        away_goals: 0,
                    });
                }
                None => Err(format!("'{value}' does not match '{pattern}'")),
            },
            Err(e) => Err(format!("build regex from '{pattern}': {e}")),
        }
    }
}

fn read_lines(file: &Path) -> Vec<String> {
    match fs::read_to_string(file) {
        Ok(s) => s.trim().split('\n').map(|s| s.trim().to_string()).collect(),
        Err(_) => Vec::new(),
    }
}

fn list_relevant_files(dir: &Path, day: Option<usize>) -> Result<Vec<PathBuf>, ()> {
    match list_files(dir) {
        Ok(files) => {
            let files_to_days: HashMap<_, _> = files
                .iter()
                .map(|f| (f, extract_day(f)))
                .map(|(p, o)| (p, o.unwrap_or(0)))
                .filter(|(_, d)| *d != 0)
                .filter(|(_, d)| day.is_none() || *d <= day.unwrap_or(usize::MAX))
                .collect();
            Ok(files_to_days.keys().map(PathBuf::from).collect())
        }
        Err(()) => Err(()),
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

fn extract_day(file: &Path) -> Option<usize> {
    let pattern = Regex::new("([0-9]+).txt$").ok()?;
    let file_name = file.to_str().unwrap_or("");
    pattern
        .captures(file_name)
        .and_then(|c| c.get(1).map(|m| m.as_str()))
        .and_then(|m| m.parse::<usize>().ok())
}
