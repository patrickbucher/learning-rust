use std::f64::consts;

trait Extent {
    fn area(&self) -> f64;
}

trait Scalable {
    fn scale(&self, factor: f64) -> impl Scalable;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Extent for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Scalable for Rectangle {
    fn scale(&self, factor: f64) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

struct Circle {
    radius: f64,
}

impl Extent for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius
    }
}

impl Scalable for Circle {
    fn scale(&self, factor: f64) -> Circle {
        Circle {
            radius: self.radius * factor,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    let circ = Circle { radius: 5.0 };
    println!("{} -> {}", rect.area(), rect.scale(2.0).area());
    println!("{} -> {}", circ.area(), circ.scale(2.0).area());

    let alice = "Alice".to_string();
    let bob = "bob";
    println!(
        "Which is longer, {alice} or {bob}? {}!",
        longest(&alice, bob)
    );
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
