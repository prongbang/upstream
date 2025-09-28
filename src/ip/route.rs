use actix_web::{get, HttpResponse, Responder};
use local_ip_address::local_ip;
use serde::Serialize;

#[derive(Serialize)]
struct IpResponse {
    ip: String,
}

#[get("/ip")]
pub async fn get_ip() -> impl Responder {
    match local_ip() {
        Ok(ip) => {
            let response = IpResponse { ip: ip.to_string() };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error retrieving IP: {}", e))
        }
    }
}