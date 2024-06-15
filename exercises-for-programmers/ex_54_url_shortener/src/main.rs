use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use md5;
use redis;
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
    cfg.route("/canary", web::get().to(canary));
    // TODO: introduce statistics page
}

async fn get_short(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let key = format!("url.{path}");
    let mut con = app_state.client.get_connection().unwrap();
    match redis::cmd("get").arg(&key).query::<String>(&mut con) {
        Ok(url) => {
            // TODO: increase counter within redis
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
    // TODO: validate URL using get request
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

async fn canary() -> impl Responder {
    format!("up and running")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let state = web::Data::new(AppState { client: client });
    let app = move || App::new().app_data(state.clone()).configure(routes);
    HttpServer::new(app).bind(BASE_URL)?.run().await
}
