use csv::Reader;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::iter;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _bin = args.next().ok_or("no binary argument")?;
    let path = args.next().ok_or("no file argument")?;
    let data = parse_csv(path)?;
    println!("{:?}", data);
    Ok(())
}

fn parse_csv(path: String) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    let header: Vec<String> = reader.headers()?.iter().map(|s| s.into()).collect();
    let mut lines: Vec<HashMap<String, String>> = Vec::new();
    for result in reader.records() {
        match result {
            Ok(record) => {
                let cols = record.iter().map(|s| s.to_string());
                let heads = header.iter().map(|s| s.to_string());
                let pairs: HashMap<String, String> = iter::zip(heads, cols).collect();
                lines.push(pairs);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(lines)
}
