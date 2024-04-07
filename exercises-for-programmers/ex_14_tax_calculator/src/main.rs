use akshually::{prompt_line, round_to};

fn main() {
    let amount: f64 = prompt_line("What is the order amount? ").expect("amount");
    let state: String = prompt_line("What is the state? ").expect("state");
    let tax = calc_tax(amount, &state);
    if tax > 0.0 {
        println!("The tax is ${:.2}.", tax);
    }
    println!("The total is ${:.2}.", amount + tax);
}

fn calc_tax(amount: f64, state: &str) -> f64 {
    if state == "WI" {
        round_to(amount * 0.055, 0.01)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_tax_wisconsin() {
        assert_eq!(calc_tax(10.0, "WI"), 0.55);
    }

    #[test]
    fn calc_tax_minnesota() {
        assert_eq!(calc_tax(10.0, "MN"), 0.0);
    }

    #[test]
    fn calc_tax_different_by_state() {
        assert_ne!(calc_tax(10.0, "WI"), calc_tax(10.0, "MN"));
    }
}
