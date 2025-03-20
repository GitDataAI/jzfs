use std::path::Path;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{http, web, HttpResponse, Responder};
use actix_web_lab::__reexports::futures_util::StreamExt;
use serde_json::json;
use tokio::io::AsyncWriteExt;
use jz_module::AppModule;
use crate::utils::session::from_session;

pub async fn upload_avatar(
    mut payload: Multipart,
    state: web::Data<AppModule>,
    session: Session,
) -> impl Responder {

    let opsuid = if let Ok(uid) = from_session(session).await {
        uid
    } else {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "no permission",
        }));
    };

    let base_dir = format!("{}/static", "./data");
    let _ = tokio::fs::create_dir_all(&base_dir).await;
    let mut avatar_path = format!("{}/{}", base_dir, opsuid);

    while let Some(Ok(mut field)) = payload.next().await {
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .unwrap_or(&format!("{}.png", opsuid))
            .to_string();

        avatar_path = format!("{}-{}", avatar_path, filename);

        let mut file = tokio::fs::File::create(&avatar_path).await.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).await.unwrap();
        }
    }

    let relative_path = format!("/api/v1/static/img/{}",
        Path::new(&avatar_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    );

    match state.user_modify_avatar(opsuid, relative_path).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": "ok",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 1,
            "msg": e.to_string(),
        }))
    }
}

pub async fn down_avatar(
    path: web::Path<String>,
) -> impl Responder {
    let filename = path.into_inner();
    let avatar_path = format!("{}/static/{}", "./data", filename);
    let mut headers = http::header::HeaderMap::new();
    headers.insert("Content-Type".parse().unwrap(), "image/png".parse().unwrap());
    match tokio::fs::read(&avatar_path).await {
        Ok(bytes) => HttpResponse::Ok()
            .content_type("image/png")
            .body(bytes),
        Err(_) => HttpResponse::NotFound()
            .body("404 Not Found")
    }
}