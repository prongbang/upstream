use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::metadata;
use std::path::Path;
use std::{env, io};

pub mod route;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileDetails {
    name: String,
    size: u64,
    r#type: String, // `type` is a reserved keyword in Rust, so we use `r#type`
    upload_date: String,
    url: String,
}

impl FileDetails {
    fn new(name: &str, size: u64, mime_type: &str, upload_date: String, url: &str) -> Self {
        FileDetails {
            name: name.to_string(),
            size,
            r#type: mime_type.to_string(),
            upload_date,
            url: url.to_string(),
        }
    }
}

pub fn get_file_details(file_path: &str) -> Result<FileDetails, Box<dyn std::error::Error>> {
    // Get the file path and metadata
    let path = Path::new(file_path);
    let metadata = metadata(path)?;

    // Extract file details
    let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default().to_string();
    let file_size = metadata.len();

    // Guess MIME type
    let mime_type = mime_guess::from_path(path).first_or(mime::APPLICATION_OCTET_STREAM);

    // Get current timestamp (upload date)
    let upload_date = Utc::now();

    // Format upload date
    let upload_date_str = upload_date.to_rfc3339();

    // Generate URL (this part can be customized based on your needs)
    let url = format!("{}", file_path);

    // Return the FileDetails struct as a response
    Ok(FileDetails::new(&file_name, file_size, &mime_type.to_string(), upload_date_str, &url))
}

pub fn get_current_working_directory() -> io::Result<String> {
    let path = env::current_dir()?;
    Ok(path.display().to_string())
}
