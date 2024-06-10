use akshually::io::prompt_line;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap();
    let input = args.next().expect(&format!("usage: {bin} FILE"));
    let output: String = prompt_line("Output file: ").unwrap();
    let content = fs::read_to_string(&input).unwrap();
    let content = content.replace("utilize", "use");
    if let Err(e) = fs::write(&output, content) {
        eprintln!("{e}");
    }
}
