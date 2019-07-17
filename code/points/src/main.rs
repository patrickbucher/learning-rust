struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let a = Point { x: 12, y: 7 };
    let b = Point { x: 3.75, y: 2.12 };
    println!("a=({}, {}), b=({}, {})", a.x, a.y, b.x, b.y);

    let c = Point { x: 10, y: 3.14 };
    println!("c=({}, {})", c.x, c.y);
}
