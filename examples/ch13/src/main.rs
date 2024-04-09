#[derive(Debug, Copy, Clone)]
enum Position {
    Engineer,
    Manager,
    Consultant,
}

struct Employee {
    name: String,
    position: Position,
    salary: f64,
}

impl Employee {
    fn show(&self) -> String {
        format!(
            "{} works as an {:?} and earns {} per year.",
            self.name, self.position, self.salary
        )
    }
}

fn main() {
    let employees = vec![
        Employee {
            name: String::from("Dilbert"),
            position: Position::Engineer,
            salary: 120000.0,
        },
        Employee {
            name: String::from("Pointy-Haired Boss"),
            position: Position::Manager,
            salary: 350000.0,
        },
        Employee {
            name: String::from("Dogbert"),
            position: Position::Consultant,
            salary: 500000.0,
        },
    ];

    for employee in employees.iter() {
        println!("{}", employee.show());
    }

    for employee in employees.iter().map(|e| Employee {
        name: e.name.clone(),
        position: e.position,
        salary: e.salary * 1.05,
    }) {
        println!("with a raise of 5%: {}", employee.show());
    }
}
