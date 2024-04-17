use std::f64::consts::PI;

pub trait Shape {
    fn circumference(&self) -> f64;

    fn area(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn area(&self) -> f64 {
        PI * self.radius.powf(2.0)
    }
}

pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Square {
        Square { side }
    }
}

impl Shape for Square {
    fn circumference(&self) -> f64 {
        4.0 * self.side
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn circumference(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}
