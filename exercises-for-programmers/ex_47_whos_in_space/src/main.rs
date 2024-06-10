use reqwest;
use serde::Deserialize;
use serde_json;

const URL: &str = "http://api.open-notify.org/astros.json";

#[derive(Deserialize)]
struct Payload {
    people: Vec<Person>,
}

#[derive(Deserialize)]
struct Person {
    craft: String,
    name: String,
}

fn main() {
    let response = reqwest::blocking::get(URL).unwrap().text().unwrap();
    let payload: Payload = serde_json::from_str(&response).unwrap();
    let title = format!("{:30} | {:15}", "Name", "Craft");
    let sep = "-".repeat(title.len());
    println!("{title}\n{sep}");
    for person in payload.people {
        println!("{:30} | {:15}", person.name, person.craft);
    }
}
