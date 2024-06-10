use akshually::io::prompt_line;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Deserialize)]
struct Product {
    name: String,
    price: f32,
    quantity: usize,
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Name: {}\n", self.name)?;
        write!(f, "Price: ${:.02}\n", self.price)?;
        write!(f, "Quantity on hand: {}", self.quantity)?;
        Ok(())
    }
}

const PRODUCTS_FILE: &str = "products.json";

fn main() {
    let raw = fs::read_to_string(PRODUCTS_FILE).expect("unable to read products file");
    let db: HashMap<String, Vec<Product>> =
        serde_json::from_str(&raw).expect("unable to deserialize products");
    let products = db.get("products").expect("no products in db");
    let query: String = prompt_line("What is the product name? ").unwrap();
    let matches: Vec<&Product> = products.iter().filter(|p| p.name == query).collect();
    if matches.is_empty() {
        println!("Sorry, that product was not found in our inventory.");
    } else {
        for product in matches {
            println!("{product}");
        }
    }
}
