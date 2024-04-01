use std::io;

const SQUARE_M_IN_SQUARE_F: f32 = 0.09290304;

fn main() {
    let mut length = String::new();
    println!("What is the length of the room in feet?");
    io::stdin().read_line(&mut length).expect("length in feet");
    let length: f32 = length.trim().parse().expect("number");

    let mut width = String::new();
    println!("What is the width of the room in feet?");
    io::stdin().read_line(&mut width).expect("width in feet");
    let width: f32 = width.trim().parse().expect("number");

    println!("You entered dimensions of {length} feet by {width} feet.");

    let area_feet = length * width;
    let area_meters = area_feet * SQUARE_M_IN_SQUARE_F;

    println!("The area is");
    println!("{area_feet} square feet");
    println!("{area_meters:.3} square meters");
}
