use akshually::io::prompt_line;
use reqwest::blocking;
use serde::Deserialize;
use std::fs::{create_dir, remove_dir_all, write};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct Payload {
    items: Vec<Item>,
    title: String,
}

#[derive(Deserialize, Debug)]
struct Item {
    media: Media,
}

#[derive(Deserialize, Debug)]
struct Media {
    m: String,
}

fn main() {
    let prompt: String = prompt_line("Enter subject for photos: ").unwrap();
    let query_string = format!("?format=json&tags={}&nojsoncallback=true", prompt);
    let url = format!(
        "https://www.flickr.com/services/feeds/photos_public.gne{}",
        query_string
    );
    let response = blocking::get(url).unwrap().text().unwrap();
    let payload: Result<Payload, _> = serde_json::from_str(&response);
    let payload = match payload {
        Err(e) => panic!("{e}"),
        Ok(p) => p,
    };
    let images: &Vec<(usize, String)> = &payload
        .items
        .iter()
        .map(|i| i.media.m.clone())
        .enumerate()
        .collect();

    let mut base_dir = PathBuf::new();
    base_dir.push(".");
    base_dir.push(prompt);
    if base_dir.exists() {
        remove_dir_all(&base_dir).unwrap();
    }
    if let Err(e) = create_dir(&base_dir) {
        panic!("{e}");
    }

    let mut html = String::from("<!DOCTYPE html><head><title>");
    html.push_str(&payload.title);
    html.push_str("</title></head><body>");
    html.push_str(&format!("<h1>{}</h1>", &payload.title));
    for (i, url) in images {
        let mut target = base_dir.clone();
        let relative_name = format!("{i}.jpg");
        target.push(&relative_name);
        let payload = blocking::get(url).unwrap().bytes().unwrap();
        write(target, payload).unwrap();
        html.push_str(&format!(
            "<span style='padding: 1em;'><img src={}></span>",
            relative_name
        ));
    }
    html.push_str("</body></html>");

    let mut html_path = base_dir.clone();
    html_path.push("index.html");
    write(html_path, html).unwrap();
}
