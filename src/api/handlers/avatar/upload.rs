use crate::STATIC_FILE;
use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::web::Path;
use actix_web::{Responder, web};
use futures_util::StreamExt;
use std::io::Write;
use uuid::Uuid;

fn save_file_create(name: String) {
    let filepath = std::path::Path::new(STATIC_FILE).join(name);
    std::fs::File::create(&filepath).ok();
}

fn save_file_add(name: String, file: web::Bytes) -> Result<(), std::io::Error> {
    let filepath = std::path::Path::new(STATIC_FILE).join(name);
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(&filepath)
        .unwrap();
    f.write_all(&file)
}

pub async fn avatar_upload(
    name: Path<String>,
    session: Session,
    meta: web::Data<crate::server::MetaData>,
    mut file: Multipart,
) -> impl Responder {
    let path = format!(
        "avatar.{}.{}.{}",
        Uuid::new_v4().to_string(),
        chrono::Local::now().timestamp(),
        name.into_inner()
    );
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(err) => return AppWrite::<String>::unauthorized(err.to_string()),
    };
    meta.users_avatar_set(model.uid, format!("/api/avatar/{}", path.clone()))
        .await
        .ok();
    save_file_create(path.clone());
    while let Some(bytes) = file.next().await {
        let mut file = match bytes {
            Ok(bytes) => bytes,
            Err(err) => return AppWrite::<String>::fail(err.to_string()),
        };
        while let Some(chunk) = file.next().await {
            if let Ok(chunk) = chunk {
                save_file_add(path.to_string(), chunk).ok();
            }
        }
    }
    AppWrite::ok(path)
}
