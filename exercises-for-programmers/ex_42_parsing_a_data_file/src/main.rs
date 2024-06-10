use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap();
    let help = format!("usage: {bin} CSV-FILE");
    let file = args.next().expect(&help);

    let content = match fs::read_to_string(&file) {
        Ok(content) => content,
        Err(err) => panic!("reading {file}: {err}"),
    };
    let lines: Vec<_> = content.split("\n").collect();
    let records: Vec<_> = lines
        .iter()
        .map(|l| l.split(",").collect())
        .filter(|l: &Vec<&str>| l.len() == 3)
        .collect();
    let last_names: Vec<&str> = records.iter().map(|r| *r.get(0).unwrap()).collect();
    let first_names: Vec<&str> = records.iter().map(|r| *r.get(1).unwrap()).collect();
    let salaries: Vec<&str> = records.iter().map(|r| *r.get(2).unwrap()).collect();

    let (last_names, last_names_len) = extend_to_max_length(&last_names);
    let (first_names, first_names_len) = extend_to_max_length(&first_names);
    let (salaries, salaries_len) = extend_to_max_length(&salaries);

    let last_title = extend_to_length("Last", last_names_len, ' ');
    let first_title = extend_to_length("First", first_names_len, ' ');
    let salary_title = extend_to_length("Salary", salaries_len, ' ');

    let title = format!("{last_title} {first_title} {salary_title}");
    let sep = "-".repeat(title.len());
    println!("{title}\n{sep}");
    for i in 0..(last_names.len()) {
        println!(
            "{} {} {}",
            last_names.get(i).unwrap(),
            first_names.get(i).unwrap(),
            salaries.get(i).unwrap()
        );
    }
}

fn extend_to_length(s: &str, len: usize, pad: char) -> String {
    let current_len = s.len();
    let diff: i32 = (len as i32) - (current_len as i32);
    if diff < 0 {
        String::from(s)
    } else {
        format!("{s}{}", String::from(pad).repeat(diff as usize))
    }
}

fn extend_to_max_length(items: &Vec<&str>) -> (Vec<String>, usize) {
    let max_len = items.iter().map(|i| i.len()).max().unwrap_or(0);
    let result: Vec<String> = items
        .iter()
        .map(|s| {
            let len = s.len();
            let diff = max_len - len;
            format!("{s}{}", " ".repeat(diff))
        })
        .collect();
    (result, max_len)
}
