use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Employee<'a> {
    first_name: &'a str,
    last_name: &'a str,
    position: &'a str,
    separation_date: &'a str,
}

impl PartialOrd for Employee<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Employee<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.last_name.cmp(other.last_name)
    }
}

impl Employee<'_> {
    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let mut employees = vec![
        Employee {
            first_name: "John",
            last_name: "Johnson",
            position: "Manager",
            separation_date: "2016-12-31",
        },
        Employee {
            first_name: "Tou",
            last_name: "Xiong",
            position: "Software Engineer",
            separation_date: "2016-10-05",
        },
        Employee {
            first_name: "Michaela",
            last_name: "Michaelson",
            position: "District Manager",
            separation_date: "2015-12-19",
        },
        Employee {
            first_name: "Jake",
            last_name: "Jacobson",
            position: "Programmer",
            separation_date: "",
        },
        Employee {
            first_name: "Jacquelyn",
            last_name: "Jackson",
            position: "DBA",
            separation_date: "",
        },
        Employee {
            first_name: "Sally",
            last_name: "Weber",
            position: "Web Developer",
            separation_date: "2015-12-18",
        },
    ];

    employees.sort();
    let title = format!(
        "{:20} | {:20} | {:20}",
        "Name", "Position", "Separation Date"
    );
    let sep = "-".repeat(title.len());
    println!("{title}");
    println!("{sep}");
    for employee in employees {
        let line = format!(
            "{:20} | {:20} | {:20}",
            employee.name(),
            employee.position,
            employee.separation_date
        );
        println!("{line}");
    }
}
