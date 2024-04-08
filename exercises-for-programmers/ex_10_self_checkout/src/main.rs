use std::io;

const TAX_RATE: f32 = 5.5;

fn main() {
    let mut subtotal: f32 = 0.0;

    for i in 1..=3 {
        println!("Enter the price of item {i}:");
        let mut price = String::new();
        io::stdin().read_line(&mut price).expect("no price entered");
        let price: f32 = price.trim().parse().expect("no numeric price entered");

        println!("Enter the quantity of item {i}:");
        let mut quantity = String::new();
        io::stdin()
            .read_line(&mut quantity)
            .expect("no quantity entered");
        let quantity: u32 = quantity
            .trim()
            .parse()
            .expect("no numeric quantity entered");

        subtotal += price * quantity as f32;
    }

    let tax = (TAX_RATE / 100.0) * subtotal as f32;
    let total = subtotal as f32 + tax;

    println!("Subtotal: ${subtotal:.2}");
    println!("Tax: ${tax:.2}");
    println!("Total: ${total:.2}");
}
