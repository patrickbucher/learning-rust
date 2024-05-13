use std::path::Path;

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Box<Vec<String>>, ()> {
    println!("dir: {:?}, day: {}", dir, day.map_or(0, |v| v));
    Ok(Box::new(Vec::new()))
}
