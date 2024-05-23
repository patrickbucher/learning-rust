use csv::Reader;
use std::env;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _bin = args.next().ok_or("no binary argument")?;
    let path = args.next().ok_or("no file argument")?;
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    for result in reader.records() {
        match result {
            Ok(record) => println!("record found: {:?}", record),
            Err(e) => eprintln!("error: {e}"),
        }
    }
    Ok(())
}
