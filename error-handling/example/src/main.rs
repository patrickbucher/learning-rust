fn main() {
    let a: usize = 9;
    let bs: &[usize] = &[0, 1, 2, 3];
    for b in bs {
        let result: Result<usize, String> = divide(a, *b).ok_or("divide by zero".to_string());
        println!("{a}/{b}={result:?}");
    }
}

fn divide(a: usize, b: usize) -> Option<usize> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
