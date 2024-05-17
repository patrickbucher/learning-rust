mod parsing;

use parsing::{list_relevant_files, read_lines, MatchResult};
use std::path::Path;

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    let mut lines: Vec<String> = Vec::new();
    if let Ok(files) = list_relevant_files(dir, day) {
        for p in files {
            lines.append(&mut read_lines(&p))
        }
    }
    for line in lines {
        let r: Result<MatchResult, _> = line.try_into();
        match r {
            Ok(m) => println!("{:?}", m),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(Box::new(Vec::new()))
}
