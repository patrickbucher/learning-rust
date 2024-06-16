use actix_web::{web, App, HttpServer};
use std::io;

const BASE_URL: &str = "127.0.0.1:8000";

struct AppState {}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let state = web::Data::new(AppState {});
    let app = move || App::new().app_data(state.clone()).configure(routes);
    HttpServer::new(app).bind(BASE_URL)?.run().await
}

fn routes(cfg: &mut web::ServiceConfig) {}
