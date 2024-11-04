mod stream;
mod ip;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::PayloadConfig;

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(PayloadConfig::default().limit(100 * 1024 * 1024 * 1024)) // 100 GB
            .service(healthcheck)
            .service(stream::route::upload)
            .service(ip::route::get_ip)
    })
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
