use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use redis;
use std::io;
use std::path::PathBuf;
use serde::Deserialize;

const REDIS_URL: &str = "redis://127.0.0.1/2";

#[derive(Deserialize)]
struct UrlForm {
    url: String,
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
    cfg.route("/url", web::post().to(post_url));
    cfg.route("/canary", web::get().to(canary));
}

async fn post_url(web::Form(form): web::Form<UrlForm>) -> impl Responder {
    format!("submitted url: {}", form.url)
}

async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn canary() -> impl Responder {
    format!("up and running")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let mut con = client.get_connection().unwrap();
    // TODO: how to pass the redis connection to the handlers?
    let app = move || App::new().configure(routes);
    HttpServer::new(app).bind("127.0.0.1:8000")?.run().await
}
