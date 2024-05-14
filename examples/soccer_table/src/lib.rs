use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    println!("dir: {:?}, day: {}", dir, day.map_or(0, |v| v));
    match list_day_files(dir, day) {
        Ok(map) => {
            for (k, v) in *map {
                println!("{k}: {v}");
            }
        },
        Err(e) => println!("~le fail: {:?}", e),
    }
    Ok(Box::new(Vec::new()))
}

fn list_day_files(dir: &Path, day: Option<usize>) -> Result<Box<HashMap<String, usize>>, ()> {
    if !dir.exists() || !dir.is_dir() {
        return Err(());
    }
    let mut days_by_file: HashMap<String, usize> = HashMap::new();
    let pattern = Regex::new("([0-9]+)").unwrap();
    if let Ok(entries) = dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().into_string().unwrap();
                let num = pattern
                    .captures(&name)
                    .and_then(|c| c.get(1))
                    .and_then(|m| Some(m.as_str()))
                    .and_then(|m| m.parse::<usize>().ok());
                if let Some(n) = num {
                    days_by_file.insert(entry.path().display().to_string(), n);
                }
            }
        }
        Ok(Box::new(days_by_file))
    } else {
        Err(())
    }
}
