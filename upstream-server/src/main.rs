mod stream;
mod ip;
mod file;
mod healthcheck;

use actix_web::web::PayloadConfig;
use actix_web::HttpServer;
use actix_web::App;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(PayloadConfig::default().limit(100 * 1024 * 1024 * 1024)) // 100 GB
            .service(healthcheck::route::healthcheck)
            .service(stream::route::forms)
            .service(stream::route::upload)
            .service(ip::route::get_ip)
            .service(file::route::get_files)
    })
        .bind(("0.0.0.0", 5000))?
        .run();

    ip::print();

    server.await
}