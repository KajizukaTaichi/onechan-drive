#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative, NamedFile};
use rocket::serde::{Serialize, json::Json};
use rocket::tokio::fs::{self, File};
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::response::status::Custom;
use std::path::Path;

#[derive(Serialize)]
struct FileInfo {
    name: String,
    path: String,
}

#[post("/upload/<file_name>", data = "<data>")]
async fn upload(file_name: &str, data: Data<'_>) -> Result<Custom<Json<FileInfo>>, Custom<String>> {
    let path = Path::new("upload").join(file_name);
    let mut _file = match File::create(&path).await {
        Ok(f) => f,
        Err(_) => return Err(Custom(Status::InternalServerError, "Failed to create file".into())),
    };

    match data.open(128.kibibytes()).into_file(&path).await {
        Ok(_) => {
            let file_info = FileInfo {
                name: file_name.into(),
                path: path.to_string_lossy().into(),
            };
            Ok(Custom(Status::Ok, Json(file_info)))
        }
        Err(_) => Err(Custom(Status::InternalServerError, "Failed to write file".into())),
    }
}

#[get("/download/<file_name>")]
async fn download(file_name: &str) -> Option<NamedFile> {
    let path = Path::new("upload").join(file_name);
    NamedFile::open(path).await.ok()
}

#[get("/files")]
async fn list_files() -> Result<Json<Vec<FileInfo>>, Custom<String>> {
    let mut entries = fs::read_dir("upload")
        .await
        .map_err(|_| Custom(Status::InternalServerError, "Failed to read directory".into()))?;

    let mut files = Vec::new();
    while let Some(entry) = entries.next_entry().await.map_err(|_| Custom(Status::InternalServerError, "Failed to read directory entry".into()))? {
        if entry.path().is_file() {
            files.push(FileInfo {
                name: entry.file_name().to_string_lossy().into(),
                path: entry.path().to_string_lossy().into(),
            });
        }
    }

    Ok(Json(files))
}

#[delete("/delete/<file_name>")]
async fn delete(file_name: &str) -> Result<Status, Custom<String>> {
    let path = Path::new("upload").join(file_name);
    fs::remove_file(path)
        .await
        .map(|_| Status::Ok)
        .map_err(|_| Custom(Status::InternalServerError, "Failed to delete file".into()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![upload, download, list_files, delete])
        .mount("/files", FileServer::from(relative!("upload")))
}
