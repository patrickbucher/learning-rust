use akshually::io::prompt_line;
use reqwest::blocking;
use std::env;

const TOKEN_VAR: &str = "OMDB_API_TOKEN";

fn main() {
    let token = match env::var(TOKEN_VAR) {
        Ok(token) => token,
        Err(err) => panic!("access env var {}: {}", TOKEN_VAR, err),
    };
    let query: String = prompt_line("Enter the name of a movie: ").unwrap();
    let response = blocking::get(format!(
        "http://www.omdbapi.com/?apikey={}&s={}",
        token, query
    )).unwrap().text().unwrap();
    println!("{:?}", response);
}
