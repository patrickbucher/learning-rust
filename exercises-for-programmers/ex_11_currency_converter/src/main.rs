use std::io;
fn main() {
    println!("How many euros are you exchanging?");
    let mut euros = String::new();
    io::stdin().read_line(&mut euros).expect("no euros entered");
    let euros: f32 = euros.trim().parse().expect("no numeric euros entered");

    println!("What is the exchange rate?");
    let mut rate_from = String::new();
    io::stdin()
        .read_line(&mut rate_from)
        .expect("no exchange rate entered");
    let rate_from: f32 = rate_from.trim().parse().expect("no numeric rate entered");

    let dollars = round_to(euros * (rate_from / 100.0), 0.01);
    println!(
        "{:.2} euros at an exchange rate of {:.2} is {:.2} U.S. dollars.",
        euros, rate_from, dollars
    );
}

fn round_to(value: f32, granularity: f32) -> f32 {
    let scale = 1.0 / granularity;
    let up = value * scale;
    let rounded = up.round();
    rounded / scale
}
