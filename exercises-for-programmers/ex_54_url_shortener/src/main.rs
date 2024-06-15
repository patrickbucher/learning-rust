use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use md5;
use redis;
use reqwest;
use serde::Deserialize;
use std::io;
use std::path::PathBuf;

const REDIS_URL: &str = "redis://127.0.0.1/2";
const BASE_URL: &str = "127.0.0.1:8000";

#[derive(Deserialize)]
struct UrlForm {
    url: String,
}

struct AppState {
    client: redis::Client,
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
    cfg.route("/url", web::post().to(post_url));
    cfg.route("/short/{digest}", web::get().to(get_short));
    cfg.route("/stats/{digest}", web::get().to(get_stats));
}

async fn get_stats(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let mut con = app_state.client.get_connection().unwrap();
    let url_key = format!("url.{path}");
    let count_key = format!("url.{path}.hits");
    let url = redis::cmd("get").arg(&url_key).query::<String>(&mut con);
    let count = redis::cmd("get").arg(&count_key).query::<usize>(&mut con);
    match (url, count) {
        (Ok(url), Ok(count)) => HttpResponse::Ok().body(format!("{path:20}{url:60}{count:10}")),
        _ => HttpResponse::InternalServerError()
            .body(format!("failed to fetch stats page for {path}")),
    }
}

async fn get_short(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let key = format!("url.{path}");
    let mut con = app_state.client.get_connection().unwrap();
    match redis::cmd("get").arg(&key).query::<String>(&mut con) {
        Ok(url) => {
            let hit_key = format!("url.{path}.hits");
            match redis::cmd("incr").arg(&hit_key).query::<usize>(&mut con) {
                Ok(x) => eprintln!("incremented {hit_key} to {x}"),
                Err(err) => eprintln!("incr {hit_key}: {err}"),
            }
            eprintln!("redirect {path} to {url}");
            HttpResponse::PermanentRedirect()
                .append_header(("Location", url))
                .finish()
        }
        Err(_) => HttpResponse::NotFound().into(),
    }
}

async fn post_url(
    app_state: web::Data<AppState>,
    web::Form(form): web::Form<UrlForm>,
) -> impl Responder {
    if let Err(err) = reqwest::get(&form.url).await {
        return HttpResponse::BadRequest().body(format!("get {}: {err}", &form.url));
    }
    let digest = md5::compute(&form.url);
    let digest = format!("{:x}", digest);
    let digest = &digest[0..6];
    let mut con = app_state.client.get_connection().unwrap();
    match redis::cmd("set")
        .arg(format!("url.{digest}"))
        .arg(&form.url)
        .query::<String>(&mut con)
    {
        Ok(_) => HttpResponse::Created().body(format!("{BASE_URL}/short/{digest}")),
        Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
    }
}

async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let state = web::Data::new(AppState { client: client });
    let app = move || App::new().app_data(state.clone()).configure(routes);
    HttpServer::new(app).bind(BASE_URL)?.run().await
}
