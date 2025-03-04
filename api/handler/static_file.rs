use crate::api::write::AppWrite;
use crate::http::GIT_ROOT;
use crate::services::AppState;
use crate::model::users::users;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{http, web, HttpResponse, Responder};
use futures_util::stream::StreamExt;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub async fn upload_avatar(
    mut payload: Multipart,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    session.renew();
    
    let uid = match session.get::<String>("user") {
        Ok(Some(uid_str)) => {
            match serde_json::from_str::<users::Model>(&uid_str) {
                Ok(user) => user.uid,
                Err(_) => return HttpResponse::Unauthorized().json(AppWrite::<()>::unauthorized("请先登录".to_string()))
            }
        }
        _ => return HttpResponse::Unauthorized().json(AppWrite::<()>::unauthorized("请先登录".to_string()))
    };

    let base_dir = format!("{}/static", GIT_ROOT); 
    let _ = tokio::fs::create_dir_all(&base_dir).await;
    let mut avatar_path = format!("{}/{}", base_dir, uid);

    while let Some(Ok(mut field)) = payload.next().await {
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .unwrap_or(&format!("{}.png", uid))
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

    match state.user_avatar_update(uid, relative_path).await {
        Ok(_) => HttpResponse::Ok().json(AppWrite::<()>::ok_msg("上传成功".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(AppWrite::<()>::error("上传失败".to_string()))
    }
}

pub async fn down_avatar(
    path: web::Path<String>,
) -> impl Responder {
    let filename = path.into_inner();
    let avatar_path = format!("{}/static/{}", GIT_ROOT, filename);
    
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
