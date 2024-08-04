use clap::Parser;
use std::fs;
use std::process;

#[derive(Debug, Parser)]
#[command(version, about)]
/// Creates random text based on an input file using the Markov Chain algorithm.
struct Args {
    /// input text
    #[arg(short, long)]
    textfile: String,

    /// prefix length
    #[arg(short, long)]
    prefix: usize,
}

fn main() {
    let args = Args::parse();
    let content = match fs::read_to_string(&args.textfile) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("reading {0}: {err}", args.textfile);
            process::exit(1);
        }
    };
    let words: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
    println!("{words:?}");
}
