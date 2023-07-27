use actix_web::{web, App, HttpResponse, HttpServer, Result};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path};

async fn read_shared_file(req: web::Path<String>) -> Result<HttpResponse> {
    // Replace "shared_folder" with the appropriate name of the mounted folder on your local machine
    let shared_folder_path = Path::new("/mnt/shared_folder");

    let file_path = shared_folder_path.join(&*req);

    match File::open(file_path) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            match file.read_to_end(&mut contents) {
                Ok(_) => Ok(HttpResponse::Ok().body(contents)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/{filename}", web::get().to(read_shared_file)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
