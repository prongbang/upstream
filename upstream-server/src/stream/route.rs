use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, HttpResponse, Responder};
use std::fs;
use std::path::Path;
use serde::Serialize;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    file: TempFile,
}

#[derive(Serialize)]
struct UploadResponse {
    path: String,
}

#[post("/up")]
pub async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let filename = form.file.file_name;
    let mut file = form.file.file;

    // Define the upload path from metadata or provide a default
    let upload_path = format!("./{}/{}", "file", &filename.unwrap());
    let path = Path::new(&upload_path);

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return HttpResponse::InternalServerError().body(format!("Error creating directories: {}", e));
        }
    }

    // Create the file at the specified path
    let mut f = match fs::File::create(&upload_path) {
        Ok(file) => file,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error creating file: {}", e)),
    };

    // Write the uploaded content to the file
    if let Err(e) = std::io::copy(&mut file, &mut f) {
        return HttpResponse::InternalServerError().body(format!("Error saving file: {}", e));
    }

    // Respond with success message
    HttpResponse::Ok().json(UploadResponse { path: upload_path })
}
