use std::env;
use std::path::Path;
use std::process;

use soccer_table::compute_table;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        eprintln!("usage: soccer-table FOLDER [DAY]");
        process::exit(1);
    }
    let _bin = args.next().unwrap();
    let dir = args.next().expect("missing DIR argument");
    let day = args.next().and_then(|day| day.parse::<usize>().ok()); // TODO: malformed -> fail!
    let _table = compute_table(Path::new(&dir), day);
}
