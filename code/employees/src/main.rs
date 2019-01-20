use std::collections::HashMap;
use std::io;

fn main() {
    let assignments = get_assignments();
    let mut employees: Vec<_> = assignments.keys().collect();
    employees.sort();
    for emp in &employees {
        if let Some(dep) = assignments.get(*emp) {
            println!("{} goes to {}.", emp, dep);
        }
    }
}

fn get_assignments() -> HashMap<String, String> {
    let mut assignments = HashMap::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().len() == 0 {
                    break;
                }
                let words: Vec<&str> = input.split_whitespace().collect();
                if words.len() != 4 || words[0] != "Add" || words[2] != "to" {
                    println!("Syntax: 'Add [name] to [department]'");
                    continue;
                }
                let name = words[1];
                let department = words[3];
                assignments.insert(name.to_string(), department.to_string());
            }
            Err(_) => break,
        }
    }
    assignments
}
