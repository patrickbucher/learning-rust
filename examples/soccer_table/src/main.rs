use std::env;
use std::path::Path;
use std::process;

use soccer_table::compute_table;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        eprintln!("usage: soccer-table DIR [DAY]");
        process::exit(1);
    }

    let _bin = args.next().unwrap();
    let dir = args.next().expect("missing DIR argument");
    let day = match args.next() {
        Some(day) => match day.parse::<usize>() {
            Ok(day) => Some(day),
            Err(e) => {
                eprintln!("parsing day '{day}': {e}");
                process::exit(1);
            }
        },
        None => None,
    };

    match compute_table(Path::new(&dir), day) {
        Ok(table) => println!("{}", table),
        Err(err) => eprintln!("{}", err),
    }
}
