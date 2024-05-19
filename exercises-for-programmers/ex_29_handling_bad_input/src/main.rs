use akshually::io::prompt_line;

fn main() {
    loop {
        match prompt_line::<f32>("What is the rate of return? ") {
            Some(0.0) => println!("Sorry. That's not a valid input."),
            Some(rate) => {
                let years = 72.0 / rate;
                println!(
                    "It will take {} years to double your initial investment.",
                    years.ceil() as i32
                );
                break;
            }
            None => println!("Sorry. That's not a valid input."),
        }
    }
}
