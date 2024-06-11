use akshually::io::prompt_line;
use reqwest::blocking;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Payload {
    main: Weather,
}

#[derive(Debug, Deserialize)]
struct Weather {
    temp: f64,
}

const TOKEN_ENVVAR: &str = "API_TOKEN";

fn main() {
    let location: String = prompt_line("Where are you? ").unwrap();
    let token = match env::var(TOKEN_ENVVAR) {
        Ok(token) => token,
        Err(err) => panic!("fetching env var {TOKEN_ENVVAR}: {err}"),
    };
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        location, token
    );
    let response = blocking::get(url).unwrap().text().unwrap();
    let response: Payload = serde_json::from_str(&response).unwrap();
    println!("{} weather:\n{} degrees Celsius", location, response.main.temp);
}
