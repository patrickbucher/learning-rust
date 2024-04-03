use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Enter \"Add [Name] to [Department]\" or \"quit\"");
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("missing input");
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        match words.as_slice() {
            &["Add", employee, "to", department] => {
                println!("Add {employee} to {department}");
                let employees: &mut Vec<String> =
                    company.entry(department.to_string()).or_insert(Vec::new());
                employees.push(employee.to_string());
            }
            &["quit"] => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
    for (department, employees) in company {
        for employee in employees {
            println!("{department}: {employee}");
        }
    }
}
