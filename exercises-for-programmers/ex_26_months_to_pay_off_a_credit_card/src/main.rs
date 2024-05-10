use akshually::io::prompt_line;

fn main() {
    let balance: f64 = prompt_line("What is your balance? ").unwrap();
    let apr: f64 = prompt_line("What is the APR on the card (as a percent)? ").unwrap();
    let monthly_payment: f64 = prompt_line("What is the monthly payment you can make? ").unwrap();

    let months = pay_off(balance, apr, monthly_payment).ceil();

    println!("It will take you {} months to pay off this card.", months);
}

fn pay_off(b: f64, apr: f64, p: f64) -> f64 {
    let i = (apr / 100.0) / 365.0;
    -(1.0 / 30.0) * ((1.0 + (b / p) * (1.0 - (1.0 + i).powf(30.0))).log10()) / (1.0 + i).log10()
}
