use crate::file::get_current_working_directory;
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use std::io;
use std::fs;

#[derive(Serialize)]
struct FileListResponse {
    files: Vec<String>,
}

fn list_files_in_directory(directory: &str) -> io::Result<Vec<String>> {
    let mut file_paths = Vec::new();

    let cur = get_current_working_directory()?;

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_path = path.display().to_string().replace("./", "");
            file_paths.push(format!("{}/{}", &cur, file_path));
        }
    }

    Ok(file_paths)
}

#[get("/file")]
pub async fn get_files() -> impl Responder {
    let directory_path = "./file";
    match list_files_in_directory(directory_path) {
        Ok(files) => HttpResponse::Ok().json(FileListResponse { files }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}