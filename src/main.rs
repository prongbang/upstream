mod file;
mod healthcheck;
mod ip;
mod stream;

use actix_files::Files;
use actix_web::web::PayloadConfig;
use actix_web::App;
use actix_web::HttpServer;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pwd = env::current_dir().expect("Failed to get current directory");
    let port = 8000;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(PayloadConfig::default().limit(100 * 1024 * 1024 * 1024)) // 100 GB
            .service(healthcheck::route::healthcheck)
            .service(stream::route::index)
            .service(stream::route::upload)
            .service(ip::route::get_ip)
            .service(file::route::get_files)
            .service(Files::new("/shared", &pwd).show_files_listing())
            .service(Files::new("/", "./web").index_file("index.html"))
    })
    .bind(("0.0.0.0", port.clone()))?
    .run();

    ip::print(port);

    server.await
}
