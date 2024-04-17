use ch17::{Circle, Rectangle, Shape, Square};

fn main() {
    let mut shapes = Vec::<Box<dyn Shape>>::new();
    shapes.push(Box::new(Circle::new(5.0)));
    shapes.push(Box::new(Square::new(3.0)));
    shapes.push(Box::new(Rectangle::new(4.0, 3.0)));

    let circumferences: Vec<f64> = shapes.iter().map(|shape| shape.circumference()).collect();
    let areas: Vec<f64> = shapes.iter().map(|shape| shape.area()).collect();

    println!("circumferences: {:?}", circumferences);
    println!("areas: {:?}", areas);
}
