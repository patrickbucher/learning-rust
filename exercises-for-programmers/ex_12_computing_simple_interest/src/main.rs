use akshually::{prompt_line, round_to};

fn main() {
    let principal: f64 = prompt_line("Enter the principal: ").expect("no principal");
    let interest: f64 = prompt_line("Enter the rate of interest: ").expect("no interest rate");
    let years: u32 = prompt_line("Enter the number of years: ").expect("no number of years");
    println!(
        "After {} years at {}%, the investment will be worth ${}.",
        years,
        interest,
        add_interest(principal, interest, years)
    );
}

fn add_interest(principal: f64, rate: f64, years: u32) -> f64 {
    round_to(principal * (1.0 + rate / 100.0 * years as f64), 0.01)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interest() {
        assert_eq!(add_interest(1500.0, 4.3, 4), 1758.0);
    }

    #[test]
    fn test_with_rounding() {
        assert_eq!(add_interest(1500.0, 1.2345, 1), 1518.52);
    }
}
