use std::collections::HashMap;

use ch16::{beavis_and_butthead, pythagoras, waste_money};

fn main() {
    beavis_and_butthead(10, 100);

    for triangle in pythagoras(vec![(2.0, 3.0), (3.0, 4.0), (4.0, 5.0)]) {
        let (a, b, c): (f32, f32, f32) = triangle;
        println!("{a:.2}²+{b:.2}²={c:.2}²");
    }

    let mut expenses = HashMap::new();
    expenses.insert(100, 0.55);
    expenses.insert(200, 1.35);
    expenses.insert(300, 2.50);
    waste_money(100.0, expenses);
}
