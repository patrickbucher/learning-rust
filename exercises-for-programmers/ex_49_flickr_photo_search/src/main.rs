use akshually::io::prompt_line;
use reqwest::blocking;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Payload {
    title: String,
}

fn main() {
    let prompt: String = prompt_line("Enter subject for photos: ").unwrap();
    println!("{prompt}");
    let query_string = format!("?format=json&tags={}&nojsoncallback=true", prompt);
    let url = format!(
        "https://www.flickr.com/services/feeds/photos_public.gne{}",
        query_string
    );
    let response = blocking::get(url).unwrap().text().unwrap();
    let payload: Result<Payload, _> = serde_json::from_str(&response);
    match payload {
        Err(e) => eprintln!("{e}"),
        Ok(p) => println!("{:?}", p),
    }
}
