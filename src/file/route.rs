use crate::file::{get_current_working_directory, get_file_details, FileDetails};
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use std::fs;
use std::io;

#[derive(Serialize)]
struct ResponseData<T> {
    code: String,
    message: String,
    data: T,
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
        Ok(files) => {
            let mut file_details: Vec<FileDetails> = vec![];
            for file in files {
                file_details.push(get_file_details(file.as_str()).unwrap());
            }

            HttpResponse::Ok().json(ResponseData {
                code: "SUC001".to_string(),
                message: "OK".to_string(),
                data: file_details,
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
