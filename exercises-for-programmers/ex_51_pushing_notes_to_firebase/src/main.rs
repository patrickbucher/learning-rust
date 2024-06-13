use chrono::{Datelike, Local};
use dotenv::dotenv;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;

const API_KEY_ENV_VAR: &str = "FIREBASE_API_KEY";
const URL: &str = "https://ex-51-notes-default-rtdb.europe-west1.firebasedatabase.app/notes.json";

#[derive(Debug, Deserialize, Serialize)]
struct Note {
    date: String,
    text: String,
}

fn main() {
    dotenv().ok();
    let (_, _token) = match env::vars().find(|(k, _)| k == API_KEY_ENV_VAR) {
        Some(token) => token,
        None => panic!("missing setting {} in .env file", API_KEY_ENV_VAR),
    };
    // NOTE: token isn't actually used, because the Firestore is public

    let mut args = env::args();
    let bin = args.next().unwrap();
    let usage = format!("usage: {bin} new TEXT|show");
    let cmd: String = match args.next() {
        Some(cmd) => cmd.to_lowercase(),
        None => panic!("{}", usage),
    };

    let client = blocking::Client::new();
    match cmd.as_str() {
        "new" => {
            let text: Vec<String> = args.collect();
            let date = Local::now().date_naive();
            let note = Note {
                date: format!("{:4}-{:02}-{:02}", date.year(), date.month(), date.day()),
                text: text.join(" "),
            };
            let body: String = serde_json::to_value(note).unwrap().to_string();
            let res = client.post(URL).body(body).send().unwrap();
            if res.status() == 200 {
                println!("Your note was saved.");
            }
        }
        "show" => {
            let payload = blocking::get(URL).unwrap().text().unwrap();
            let notes: HashMap<String, Note> = serde_json::from_str(&payload).unwrap();
            for (_, note) in notes {
                println!("{} - {}", note.date, note.text);
            }
        }
        _ => panic!("{}", usage),
    }
}
