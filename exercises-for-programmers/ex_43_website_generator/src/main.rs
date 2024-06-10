use akshually::io::prompt_line;
use std::fs;
use std::path::PathBuf;

fn main() {
    let site_name: String = prompt_line("Site name: ").unwrap();
    let author: String = prompt_line("Author: ").unwrap();
    let js_folder: String = prompt_line("Do you want a folder for JavaScript? ")
        .unwrap_or(String::from("n"))
        .trim()
        .to_lowercase();
    let css_folder: String = prompt_line("Do you want a folder for CSS? ")
        .unwrap_or(String::from("n"))
        .trim()
        .to_lowercase();

    match scaffold(&site_name, &author, js_folder == "y", css_folder == "y") {
        Ok(artifacts) => {
            for artifact in artifacts {
                println!("Created {artifact}");
            }
        }
        Err(ScaffoldError::FileSystem(err)) => eprintln!("{:?}", err),
    }
}

#[derive(Debug)]
enum ScaffoldError {
    FileSystem(String),
}

fn scaffold(name: &str, author: &str, js: bool, css: bool) -> Result<Vec<String>, ScaffoldError> {
    let mut artifacts: Vec<String> = Vec::new();

    let mut base_dir = PathBuf::new();
    base_dir.push(".");
    base_dir.push(name);
    fs::create_dir(&base_dir).map_err(|e| ScaffoldError::FileSystem(format!("base dir: {e}")))?;
    artifacts.push(format!("{}", base_dir.display()));

    if js {
        let mut js_dir = base_dir.clone();
        js_dir.push("js");
        fs::create_dir(&js_dir).map_err(|e| ScaffoldError::FileSystem(format!("js dir: {e}")))?;
        artifacts.push(format!("{}", js_dir.display()));
    }

    if css {
        let mut css_dir = base_dir.clone();
        css_dir.push("css");
        fs::create_dir(&css_dir).map_err(|e| ScaffoldError::FileSystem(format!("css dir: {e}")))?;
        artifacts.push(format!("{}", css_dir.display()));
    }

    let html = format!("<!DOCTYPE html>\n<html><head><title>{name}</title>");
    let html = format!("{html}<meta author=\"{author}\"></head><body></body></html>");
    let mut html_file = base_dir.clone();
    html_file.push("index.html");
    fs::write(&html_file, html)
        .map_err(|e| ScaffoldError::FileSystem(format!("html file: {e}")))?;
    artifacts.push(format!("{}", html_file.display()));

    Ok(artifacts)
}
