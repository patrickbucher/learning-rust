use akshually::io::prompt_line;

fn main() {
    let amount: f32 = prompt_line("What is the order amount? ").unwrap();
    let state: String = prompt_line("What state do you live in? ").unwrap();
    let tax_rate = match state.as_str() {
        "Wisconsin" => {
            let county: String = prompt_line("County: ").unwrap();
            let surplus = match county.as_str() {
                "Eau Claire" => 0.005,
                "Dunn" => 0.004,
                _ => 0.0,
            };
            0.08 + surplus
        }
        "Illinois" => 0.08,
        _ => 0.0,
    };
    let tax = amount * tax_rate;
    let total = amount + tax;
    println!("The tax is ${tax:.2}.");
    println!("The total is ${total:.2}.");
}
