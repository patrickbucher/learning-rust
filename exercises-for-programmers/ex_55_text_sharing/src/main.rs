use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use md5;
use redis;
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::PathBuf;

const BASE_URL: &str = "127.0.0.1:8000";
const REDIS_URL: &str = "redis://127.0.0.1/3";

struct AppState {
    client: redis::Client,
}

impl AppState {
    fn set_text(&self, text: &str, hash: Option<&str>) -> Result<String, String> {
        let hash = match hash {
            Some(hash) => String::from(hash),
            None => {
                let hash = md5::compute(text);
                format!("{:x}", hash)
            }
        };
        let key = format!("text.{hash}");
        let mut con = self.client.get_connection().unwrap();
        match redis::cmd("set")
            .arg(&key)
            .arg(&text)
            .query::<String>(&mut con)
        {
            Ok(_) => Ok(hash),
            Err(err) => Err(format!("set {key}: {err}")),
        }
    }

    fn get_text(&self, hash: &str) -> Result<String, String> {
        let mut con = self.client.get_connection().unwrap();
        let key = format!("text.{hash}");
        match redis::cmd("get").arg(&key).query::<String>(&mut con) {
            Ok(text) => Ok(text),
            Err(err) => Err(format!("get {}: {err}", key)),
        }
    }
}

#[derive(Deserialize)]
struct TextForm {
    text: String,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let state = web::Data::new(AppState {
        client: redis::Client::open(REDIS_URL).unwrap(),
    });
    let app = move || App::new().app_data(state.clone()).configure(routes);
    HttpServer::new(app).bind(BASE_URL)?.run().await
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(get_index));
    cfg.route("/text", web::post().to(post_text));
    cfg.route("/text/{hash}", web::get().to(get_text));
    cfg.route("/edit/{hash}", web::get().to(get_edit));
    cfg.route("/edit/{hash}", web::post().to(post_edit));
}

async fn get_index() -> Result<NamedFile> {
    let path: PathBuf = "./index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn post_text(
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<TextForm>,
) -> impl Responder {
    match app_state.set_text(&form.text, None) {
        Ok(hash) => HttpResponse::Created().body(format!("{BASE_URL}/text/{hash}")),
        Err(err) => HttpResponse::InternalServerError().body(format!("error: {err}")),
    }
}

async fn get_text(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let hash = format!("{path}");
    load_text(app_state, &hash, "./text.html")
}

async fn get_edit(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let hash = format!("{path}");
    load_text(app_state, &hash, "./edit.html")
}

fn load_text(app_state: web::Data<AppState>, hash: &str, template_file: &str) -> impl Responder {
    match app_state.get_text(&hash) {
        Ok(text) => {
            let template = fs::read_to_string(template_file).unwrap();
            let html = template
                .replace("{{HASH}}", &hash)
                .replace("{{TEXT}}", &text);
            HttpResponse::Ok().body(html)
        }
        Err(err) => HttpResponse::NotFound().body(format!("no text with hash {hash}: {err}")),
    }
}

async fn post_edit(
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<TextForm>,
    path: web::Path<String>,
) -> impl Responder {
    let hash = format!("{path}");
    match app_state.set_text(&form.text, Some(&hash)) {
        Ok(hash) => HttpResponse::Ok().body(format!("{BASE_URL}/text/{hash}")),
        Err(err) => HttpResponse::InternalServerError().body(format!("error: {err}")),
    }
}
