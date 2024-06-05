use akshually::io::prompt_line;

fn main() {
    let employees = vec![
        "John Smith",
        "Jackie Jackson",
        "Chris Jones",
        "Amanda Cullen",
        "Jeremy Goodwin",
    ];
    for employee in &employees {
        println!("{employee}");
    }
    let employee: String = prompt_line("Enter an employee name to remove: ").unwrap();
    let employees = employees.iter().filter(|&e| *e != employee);
    for employee in employees {
        println!("{employee}");
    }
}
