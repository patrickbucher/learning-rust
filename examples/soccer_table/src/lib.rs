use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    println!("dir: {:?}, day: {}", dir, day.map_or(0, |v| v));
    match list_day_files(dir, day) {
        Ok(_) => println!("~le ok"),
        Err(e) => println!("~le fail: {:?}", e),
    }
    Ok(Box::new(Vec::new()))
}

fn list_day_files(dir: &Path, day: Option<usize>) -> Result<Box<HashMap<String, usize>>, ()> {
    if !dir.exists() || !dir.is_dir() {
        return Err(());
    }
    let entries: HashMap<String, usize> = HashMap::new();
    let pattern = Regex::new("day([0-9]+)").unwrap();
    if let Ok(entries) = dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().into_string().unwrap();
                let tmp = pattern
                    .captures(&name)
                    .and_then(|c| c.get(1))
                    .and_then(|m| Some(m.as_str().parse::<usize>()));
                println!("{}: {}", entry.path().display(), tmp.unwrap().unwrap());
            }
        }
        Ok(Box::new(HashMap::new()))
    } else {
        Err(())
    }
}
