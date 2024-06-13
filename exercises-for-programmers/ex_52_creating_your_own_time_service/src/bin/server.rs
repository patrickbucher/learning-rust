use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, Local, Timelike};
use std::io;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/time", web::get().to(get_time));
}

pub async fn get_time() -> impl Responder {
    let now = Local::now();
    HttpResponse::Ok().json(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    ))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(routes);
    HttpServer::new(app).bind("127.0.0.1:8000")?.run().await
}
