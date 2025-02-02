use crate::STATIC_FILE;
use actix_web::{HttpResponse, Responder, web};
use std::fs;

pub async fn avatar_download(path: web::Path<String>) -> impl Responder {
    let path = path.into_inner();
    let file_path = format!("{}/{}", STATIC_FILE, path);

    match fs::read(file_path) {
        Ok(bytes) => HttpResponse::Ok().content_type("image/jpeg").body(bytes),
        Err(_) => HttpResponse::NotFound().body("Image not found"),
    }
}
