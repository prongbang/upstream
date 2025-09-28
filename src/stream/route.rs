use actix_multipart::Multipart;
use actix_web::{get, post, HttpResponse, Responder};
use futures_util::StreamExt;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
struct UploadResponse {
    pub paths: Vec<String>,
}

// Embed the HTML file at compile time
const INDEX_HTML: &str = include_str!("../.././web/index.html");

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(INDEX_HTML)
}

#[post("/up")]
pub async fn upload(mut payload: Multipart) -> impl Responder {
    // Define the upload directory
    let upload_dir = "./file";
    let mut upload_paths = Vec::new(); // To store the paths of uploaded files

    // Loop over each field in the Multipart form data
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let file_name = field
            .content_disposition()
            .expect("REASON")
            .get_filename()
            .unwrap_or("default_file_name")
            .to_string();

        // Prepare the file path
        let upload_path = Path::new(upload_dir).join(&file_name);

        // Create the upload directory if it doesn't exist
        if let Some(parent) = upload_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return HttpResponse::InternalServerError()
                    .body(format!("Error creating directories: {}", e));
            }
        }

        // Create the file where data will be saved
        let mut f = match File::create(&upload_path) {
            Ok(file) => file,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error creating file: {}", e))
            }
        };

        // Stream the file data from the multipart field to the disk
        while let Some(Ok(chunk)) = field.next().await {
            if let Err(e) = f.write_all(&chunk) {
                return HttpResponse::InternalServerError()
                    .body(format!("Error writing to file: {}", e));
            }
        }

        // Add the path to the list of uploaded files
        upload_paths.push(upload_path.to_string_lossy().to_string());
    }

    // If no files were uploaded, return a BadRequest response
    if upload_paths.is_empty() {
        return HttpResponse::BadRequest().body("No files uploaded.");
    }

    // Respond with the paths of the uploaded files
    HttpResponse::Ok().json(UploadResponse {
        paths: upload_paths,
    })
}
