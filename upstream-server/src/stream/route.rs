use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, HttpResponse, Responder};
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

#[get("/")]
pub async fn forms() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
          <meta charset="UTF-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <title>UPATREAM</title>
          <style>
            /* Styles remain the same */
            * { box-sizing: border-box; margin: 0; padding: 0; }
            body { font-family: Arial, sans-serif; display: flex; align-items: center; justify-content: center; min-height: 100vh; padding: 20px; background-color: #f4f4f4; }
            .container { width: 100%; max-width: 400px; background: #fff; padding: 20px; border-radius: 8px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); }
            h2 { text-align: center; margin-bottom: 20px; }
            form { display: flex; flex-direction: column; gap: 15px; }
            input[type="file"] { padding: 10px; }
            button { padding: 10px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 1em; }
            button:hover { background-color: #0056b3; }
            #progressContainer { display: none; margin-top: 15px; }
            #progressWrapper { width: 100%; background-color: #ddd; border-radius: 4px; }
            #progressBar { width: 0; height: 20px; background-color: #4caf50; border-radius: 4px; transition: width 0.3s; }
            #progressText { margin-top: 5px; text-align: center; }
            #successDialog { display: none; padding: 20px; text-align: center; background-color: #e7ffe7; border: 1px solid #4caf50; border-radius: 8px; margin-top: 15px; }
            @media (max-width: 500px) { .container { padding: 15px; } button { font-size: 0.9em; } }
            .mt-2 { margin-top: 1rem; }
          </style>
        </head>
        <body>
          <div class="container">
            <h2>UPATREAM</h2>
            <form id="uploadForm" enctype="multipart/form-data">
              <input type="file" id="fileInput" name="file" required>
              <button type="submit">UP</button>
            </form>

            <div id="progressContainer">
              <div id="progressWrapper">
                <div id="progressBar"></div>
              </div>
              <p id="progressText">0%</p>
            </div>

            <div id="successDialog">
              <p>File uploaded successfully!</p>
              <button class="mt-2" onclick="closeDialog()">Close</button>
            </div>
          </div>

          <script>
            document.getElementById('uploadForm').addEventListener('submit', function(event) {
              event.preventDefault();

              const fileInput = document.getElementById('fileInput');
              const file = fileInput.files[0];
              if (!file) return;

              const formData = new FormData();
              formData.append('file', file);

              const xhr = new XMLHttpRequest();
              const progressContainer = document.getElementById('progressContainer');
              const progressBar = document.getElementById('progressBar');
              const progressText = document.getElementById('progressText');
              progressContainer.style.display = 'block';

              // Set up progress monitoring
              xhr.upload.onprogress = function(event) {
                if (event.lengthComputable) {
                  const percentComplete = (event.loaded / event.total) * 100;
                  progressBar.style.width = `${percentComplete}%`;
                  progressText.innerText = `${Math.round(percentComplete)}%`;
                }
              };

              // Handle response after upload is complete
              xhr.onload = function() {
                if (xhr.status === 200) {
                  progressBar.style.width = '100%';
                  progressText.innerText = '100%';
                  progressContainer.style.display = 'none';
                  document.getElementById('successDialog').style.display = 'block';
                } else {
                  console.error('Upload failed:', xhr.statusText);
                }
              };

              xhr.onerror = function() {
                console.error('An error occurred during the upload.');
              };

              xhr.open('POST', '/up', true);
              xhr.send(formData);
            });

            function closeDialog() {
              document.getElementById('successDialog').style.display = 'none';
            }
          </script>
        </body>
        </html>
        "#,
    )
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
