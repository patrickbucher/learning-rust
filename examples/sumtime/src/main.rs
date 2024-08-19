use std::io;
use sumtime::TimeParser;

fn main() {
    let mut lines: Vec<String> = Vec::new();
    let mut buf = String::new();
    let stdin = io::stdin();
    while let Ok(n) = stdin.read_line(&mut buf) {
        if n > 1 {
            lines.push(String::from(buf.trim()));
            buf = String::new();
        } else {
            break;
        }
    }
    let parser = TimeParser::new();
    let mut hh_mm: Vec<(usize, usize)> = Vec::new();
    for line in lines {
        hh_mm.push(parser.parse(&line));
    }
    let (mut hh, mut mm) = hh_mm
        .iter()
        .fold((0, 0), |(hh, mm), (h, m)| ((hh + h), (mm + m)));
    hh += mm / 60;
    mm %= 60;
    println!("{hh}:{mm:02}");
}
