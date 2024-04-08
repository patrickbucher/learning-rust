use akshually::{prompt_line, round_to};

fn main() {
    let principal: f64 = prompt_line("What is the principal amount? ").expect("principal");
    let rate: f64 = prompt_line("What is the rate? ").expect("rate");
    let years: u32 = prompt_line("What is the number of years? ").expect("years");
    let compounds_per_year: u8 =
        prompt_line("What is the number of times the interest is compounded per year? ")
            .expect("compounds per year");
    println!(
        "${} invested at {}% for {} years compounded {} times per year is ${}.",
        principal,
        rate,
        years,
        compounds_per_year,
        compound_interest(principal, rate, years, compounds_per_year)
    );
}

fn compound_interest(principal: f64, rate: f64, years: u32, compounds_per_year: u8) -> f64 {
    let base: f64 = 1.0 + (rate / 100.0) / compounds_per_year as f64;
    let exp = compounds_per_year as u32 * years;
    round_to(principal * base.powf(exp as f64), 0.01)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compound_interest() {
        assert_eq!(compound_interest(1500.0, 4.3, 6, 4), 1938.84);
    }
}
