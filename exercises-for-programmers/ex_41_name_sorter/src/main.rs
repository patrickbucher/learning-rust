use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap();
    let usage = format!("usage: {bin} IN-FILE OUT-FILE");
    let in_file = args.next().expect(&usage);
    let out_file = args.next().expect(&usage);

    let content = match fs::read_to_string(&in_file) {
        Ok(content) => content,
        Err(err) => panic!("reading file {in_file}: {err}"),
    };
    let mut lines: Vec<&str> = content
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| *l != "")
        .collect();
    lines.sort();

    let mut handle = match fs::File::create(&out_file) {
        Ok(handle) => handle,
        Err(err) => panic!("creating file {out_file}: {err}"),
    };

    let title = format!("Table of {} names", lines.len());
    let separator = "-".repeat(title.len());
    let mut output = format!("{title}\n{separator}");
    for line in lines {
        output = format!("{output}\n{line}");
    }
    if let Err(err) = handle.write(&output.into_bytes()) {
        panic!("writing file {out_file}: {err}");
    }
}
