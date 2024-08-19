use regex::Regex;
use std::io;

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
    let re = Regex::new("^([0-9]+):([0-5][0-9])$").unwrap();
    let mut total_hours: usize = 0;
    let mut total_minutes: usize = 0;
    for line in lines {
        match re.captures(&line) {
            Some(matches) => {
                if matches.len() == 3 {
                    let (hours, minutes) = match (matches.get(1), matches.get(2)) {
                        (Some(h), Some(m)) => (h.as_str(), m.as_str()),
                        _ => ("0", "0"),
                    };
                    let (hours, minutes): (usize, usize) =
                        (hours.parse().unwrap_or(0), minutes.parse().unwrap_or(0));
                    total_hours += hours;
                    total_minutes += minutes;
                } else {
                    continue;
                }
            }
            None => continue,
        }
    }
    total_hours += total_minutes / 60;
    total_minutes %= 60;
    println!("{total_hours}:{total_minutes:02}");
}
