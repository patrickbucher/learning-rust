use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    println!("dir: {:?}, day: {}", dir, day.map_or(0, |v| v));
    match list_files(dir) {
        Ok(vec) => {
            for v in vec {
                println!("{}", v.display());
            }
        }
        Err(e) => println!("~le fail: {:?}", e),
    }
    Ok(Box::new(Vec::new()))
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
    let pattern = Regex::new("([0-9]+)").ok()?;
    let file_name = file.as_path().to_str().unwrap_or("".into());
    pattern
        .captures(&file_name)
        .and_then(|c| c.get(1))
        .and_then(|m| Some(m.as_str()))
        .and_then(|m| Some(m.parse::<usize>().ok()?))
}

fn list_day_files(dir: &Path) -> Result<HashMap<String, usize>, ()> {
    if !dir.exists() || !dir.is_dir() {
        return Err(());
    }
    let mut days_by_file: HashMap<String, usize> = HashMap::new();
    let pattern = Regex::new("([0-9]+)").or(Err(()))?;
    let entries = dir.read_dir().or(Err(()))?;
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap_or("".into());
            let num = pattern
                .captures(&file_name)
                .and_then(|c| c.get(1))
                .and_then(|m| Some(m.as_str()))
                .and_then(|m| m.parse::<usize>().ok());
            if let Some(n) = num {
                days_by_file.insert(entry.path().display().to_string(), n);
            }
        }
    }
    Ok(days_by_file)
}
