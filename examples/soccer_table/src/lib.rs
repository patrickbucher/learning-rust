use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    println!("dir: {:?}, day: {}", dir, day.map_or(0, |v| v));
    for s in list_relevant_files(dir, day) {
        println!("{}", s);
    }
    Ok(Box::new(Vec::new()))
}

fn list_relevant_files(dir: &Path, day: Option<usize>) -> Vec<String> {
    match list_files(dir) {
        Ok(files) => {
            let files_to_days: HashMap<_, _> = files
                .iter()
                .map(|f| (f.as_path(), extract_day(&f)))
                .map(|(p, o)| (p.to_str().unwrap_or(""), o.unwrap_or(0)))
                .filter(|(s, d)| *s != "" && *d != 0)
                .filter(|(s, d)| day.is_none() || *d <= day.unwrap_or(usize::MAX))
                .collect();
            files_to_days.keys().map(|s| s.to_string()).collect()
        }
        Err(()) => Vec::new(),
    }
}

fn list_files(dir: &Path) -> Result<Vec<PathBuf>, ()> {
    if !dir.exists() || !dir.is_dir() {
        return Err(());
    }
    let mut files: Vec<_> = Vec::new();
    let entries = dir.read_dir().or(Err(()))?;
    for entry in entries {
        if let Ok(e) = entry {
            if let Ok(t) = e.file_type() {
                if t.is_file() {
                    files.push(dir.join(e.path()));
                }
            }
        }
    }
    Ok(files)
}

fn extract_day(file: &PathBuf) -> Option<usize> {
    let pattern = Regex::new("([0-9]+).txt$").ok()?;
    let file_name = file.as_path().to_str().unwrap_or("".into());
    pattern
        .captures(&file_name)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .and_then(|m| Some(m.parse::<usize>().ok()?))
}
