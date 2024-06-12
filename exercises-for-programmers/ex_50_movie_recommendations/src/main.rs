use akshually::io::prompt_line;
use reqwest::blocking;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
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

#[derive(Debug, Deserialize)]
struct Movie {
    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "Year")]
    year: String,

    #[serde(rename = "Rated")]
    rated: String,

    #[serde(rename = "Runtime")]
    runtime: String,

    #[serde(rename = "Plot")]
    plot: String,

    #[serde(rename = "Ratings")]
    ratings: Vec<Rating>,
}

#[derive(Debug, Deserialize)]
struct Rating {
    #[serde(rename = "Source")]
    source: String,

    #[serde(rename = "Value")]
    value: String,
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
    let payload: Payload = serde_json::from_str(&response).unwrap();

    let indexed_results: Vec<(usize, &SearchResult)> = payload.search.iter().enumerate().collect();
    for (i, result) in indexed_results {
        println!("{i}: {} ({})", result.title, result.year);
    }
    let index: usize = prompt_line::<String>("Pick a movie by index: ")
        .unwrap()
        .parse()
        .unwrap();
    let indexed_results: HashMap<usize, &SearchResult> =
        payload.search.iter().enumerate().collect();
    let movie = match indexed_results.get(&index) {
        Some(result) => result,
        None => panic!("No result with index {index}"),
    };

    let response = blocking::get(format!(
        "http://www.omdbapi.com/?apikey={}&i={}",
        token, movie.imdb_id
    ))
    .unwrap()
    .text()
    .unwrap();
    let movie: Movie = serde_json::from_str(&response).unwrap();

    println!("Title: {}", movie.title);
    println!("Year: {}", movie.year);
    println!("Rating: {}", movie.rated);
    println!("Running Time: {}", movie.runtime);
    println!("\nDescription: {}\n", movie.plot);

    match movie.ratings.iter().find(|r| r.source == "Rotten Tomatoes") {
        Some(rating) => {
            let score: usize = rating.value.replace("%", "").parse().unwrap();
            if score > 80 {
                println!("You should watch this movie right now!");
            } else if score < 50 {
                println!("Avoid this movie at all cost!");
            }
        }
        None => println!("No rating found."),
    }
}
