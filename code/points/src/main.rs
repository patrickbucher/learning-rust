struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,  // T
            y: other.y, // W
        }
    }
}

fn main() {
    let a = Point { x: 1, y: 2.5 };
    let b = Point { x: 3.99, y: 8 };
    let p = a.mixup(b);
    println!("p=({},{})", p.x, p.y);
}
