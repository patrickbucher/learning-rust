use reqwest::blocking;

fn main() {
    let now = blocking::get("http://127.0.0.1:8000/time")
        .unwrap()
        .text()
        .unwrap();
    println!("The current time is {}", now);
}
