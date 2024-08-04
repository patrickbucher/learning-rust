use clap::Parser;
use markov_chain::build_markov_chain;
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

    /// output length
    #[arg(short, long)]
    length: usize,
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
    let generated = build_markov_chain(&content, args.prefix, args.length);
    println!("{generated}");
}
