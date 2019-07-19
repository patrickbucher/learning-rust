fn main() {
    let result: &str;
    let a = String::from("foobar");
    {
        let b = String::from("qux");
        result = longest(a.as_str(), b.as_str());
        println!("The longest string is '{}'", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
