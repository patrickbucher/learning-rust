use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use md5;
use redis;
use serde::Deserialize;
use std::io;
use std::path::PathBuf;

const BASE_URL: &str = "127.0.0.1:8000";
const REDIS_URL: &str = "redis://127.0.0.1/3";

struct AppState {
    client: redis::Client,
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
}

async fn get_index() -> Result<NamedFile> {
    let path: PathBuf = "./index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn post_text(
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<TextForm>,
) -> impl Responder {
    let hash = md5::compute(&form.text);
    let hash = format!("{hash:x}");
    let key = format!("text.{hash}");
    eprintln!("post text: {} ({})", &form.text, hash);

    let mut con = app_state.client.get_connection().unwrap();
    match redis::cmd("set")
        .arg(key)
        .arg(&form.text)
        .query::<String>(&mut con)
    {
        Ok(_) => HttpResponse::Created().body(format!("{BASE_URL}/text/{hash}")),
        Err(err) => HttpResponse::InternalServerError().body(format!("error: {err}")),
    }
}
