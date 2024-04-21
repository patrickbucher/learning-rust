use akshually::io::prompt_line;

fn main() {
    println!("Press C to convert from Fahrenheit to Celsius.");
    println!("Press F to convert from Celsius to Fahrenheit.");
    loop {
        let choice: char = prompt_line("Your choice: ").expect("'C' or 'F'");
        match choice {
            'C' => {
                let fahr: f64 = prompt_line("Please enter the temperature in Fahrenheit: ")
                    .expect("temperature in Fahrenheit");
                let cels = (fahr - 32.0) * 5.0 / 9.0;
                println!("The temperature in Celsius is {}.", cels);
            }
            'F' => {
                let cels: f64 = prompt_line("Please enter the temperature in Celsius: ")
                    .expect("temperature in Fahrenheit");
                let fahr = (cels * 9.0 / 5.0) + 32.0;
                println!("The temperature in Fahrenheit is {}.", fahr);
            }
            _ => continue,
        }
    }
}
