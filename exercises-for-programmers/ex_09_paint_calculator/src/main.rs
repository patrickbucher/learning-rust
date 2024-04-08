use std::io;

const GALLONS_PER_SQUARE_FEET: u32 = 350;

fn main() {
    let mut length = String::new();
    println!("What's the length of the room to be painted?");
    io::stdin().read_line(&mut length).expect("no length given");
    let length: u32 = length.trim().parse().expect("no numeric length given");

    let mut width = String::new();
    println!("What's the width of the room to be painted?");
    io::stdin().read_line(&mut width).expect("no width given");
    let width: u32 = width.trim().parse().expect("no numeric width given");

    let square_feet = length * width;
    let gallons = (square_feet as f32 / GALLONS_PER_SQUARE_FEET as f32).ceil();

    println!(
        "You will need to purchase {} gallons of paint to cover {} square feet.",
        gallons, square_feet
    );
}
