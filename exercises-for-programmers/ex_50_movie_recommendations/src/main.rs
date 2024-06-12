use akshually::io::prompt_line;
use reqwest::blocking;
use serde::Deserialize;
use serde_json;
use std::env;

const TOKEN_VAR: &str = "OMDB_API_TOKEN";

#[derive(Debug, Deserialize)]
struct Payload {
    #[serde(rename = "Search")]
    search: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "Year")]
    year: String,

    #[serde(rename = "imdbID")]
    imdb_id: String,
}

fn main() {
    let token = match env::var(TOKEN_VAR) {
        Ok(token) => token,
        Err(err) => panic!("access env var {}: {}", TOKEN_VAR, err),
    };
    let query: String = prompt_line("Enter the name of a movie: ").unwrap();
    let response = blocking::get(format!(
        "http://www.omdbapi.com/?apikey={}&s={}",
        token, query
    ))
    .unwrap()
    .text()
    .unwrap();
    println!("{:?}", response);
    let payload: Payload = serde_json::from_str(&response).unwrap();
    println!("{:?}", payload);

    // TODO: store HashMap of SearchResult by arbitrary index (enumerate)
    // TODO: display menu with all Titles/Years and index
    // TODO: allow the user to pick an index
    // TODO: query the API by imdbID
}
