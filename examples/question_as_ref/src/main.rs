fn divide(a: f64, b: f64) -> Result<Success, Failure> {
    match b {
        0.0 => Err(Failure::Why(format!("divide by zero: {a}/{b}"))),
        _ => Ok(Success { result: a / b }),
    }
}

#[derive(Debug)]
struct Success {
    result: f64,
}

#[derive(Debug)]
enum Failure {
    Why(String),
}

fn main() {
    let divisions = [(9.0, 3.0), (7.0, 0.0), (12.0, 4.0)];
    let (worked, failed): (Vec<Result<Success, Failure>>, Vec<Result<Success, Failure>>) =
        divisions
            .iter()
            .map(|d| divide(d.0, d.1))
            .partition(|e| e.is_ok());
    /*
    let errors: String = failed
        .iter()
        .map(|f| f.unwrap_err())
        .map(|f| format!("{:?}", f))
        .collect::<Vec<_>>()
        .join(", ");
    println!("errors: {}", errors);
    */

    let mut iter: std::slice::Iter<Result<Success, Failure>> = failed.iter();
    let item: &Result<Success, Failure> = iter.next().unwrap();
}

