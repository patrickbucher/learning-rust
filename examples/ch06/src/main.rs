use std::f32::consts;

#[derive(Debug)]
enum Shape {
    Square(f32),
    Rectangle(f32, f32),
    Circle(f32),
}

fn main() {
    let square = Shape::Square(4.0);
    let rect = Shape::Rectangle(3.0, 4.0);
    let circle = Shape::Circle(2.5);

    println!("shapes: {:?} {:?} {:?}", square, rect, circle);
    println!(
        "circumferences: {:?} {:?} {:?}",
        circumference(&square),
        circumference(&rect),
        circumference(&circle)
    );
    println!(
        "areas: {:?} {:?} {:?}",
        area(&square),
        area(&rect),
        area(&circle)
    );

    println!("3/0={:?}", divide(3, 0));
    println!("12/3={:?}", divide(12, 3));

    if let Some(quota) = divide(9, 3) {
        println!("9/3={}", quota);
    }
}

fn circumference(shape: &Shape) -> f32 {
    match shape {
        Shape::Square(side) => side * 4.0,
        Shape::Rectangle(width, height) => 2.0 * (width + height),
        Shape::Circle(radius) => 2.0 * radius * consts::PI,
    }
}

fn area(shape: &Shape) -> f32 {
    match shape {
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
        Shape::Circle(radius) => consts::PI * radius * radius,
    }
}

fn divide(a: u32, b: u32) -> Option<u32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
