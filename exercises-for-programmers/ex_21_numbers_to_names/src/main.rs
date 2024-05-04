use akshually::io::prompt_line;

fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let month_number: usize = prompt_line("Please enter the number of the month: ").unwrap();
    if (1..=12).contains(&month_number) {
        let index = month_number - 1;
        println!("The name of the month is {}.", months[index]);
    } else {
        eprintln!("{month_number} is not a valid month number.")
    }
}
