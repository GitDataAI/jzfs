use std::collections::HashMap;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use time::format_description;
use crate::metadata::service::MetaService;
use crate::ROOT_PATH;

pub async fn objects_pack(http_request: HttpRequest, path: web::Path<(String, String, String)>, service: web::Data<MetaService>) -> impl Responder {
    let (owner, repo_name,pack_hash) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name.clone()).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let path = format!("objects/pack/{}", pack_hash);
    let url = http_request.uri().to_string().replace(&repo_path.clone(), "");

    let mut map = HashMap::new();
    let time = time::OffsetDateTime::now_utc();
    let expires = time::OffsetDateTime::now_utc() + time::Duration::days(1);
    map.insert("Date".to_string(), time.format(&format_description::parse("%a, %d %b %Y %H:%M:%S GMT").unwrap()).unwrap());
    map.insert("Expires".to_string(), expires.format(&format_description::parse("%a, %d %b %Y %H:%M:%S GMT").unwrap()).unwrap());
    map.insert("Cache-Control".to_string(), "public, max-age=86400".to_string());
    #[allow(unused_assignments)]
    let mut xtype = "application/x-git-loose-object".to_string();
    if url.ends_with(".pack") {
        xtype = "application/x-git-packed-objects".to_string();
    } else if url.ends_with(".idx") {
        xtype = "application/x-git-packed-objects-toc".to_string();
    } else {
        xtype = "application/x-git-loose-object".to_string();
    }

    let req_file = PathBuf::from(ROOT_PATH).join(repo_path).join(path);
    if !req_file.exists() {
        return HttpResponse::NotFound().body("File not found");
    }
    match NamedFile::open(req_file) {
        Ok(mut named_file) => {
            named_file = named_file.use_last_modified(true);
            let mut response = named_file.into_response(&http_request);
            for (k, v) in map.iter() {
                response.headers_mut().insert(
                    k.to_string().parse().unwrap(),
                    HeaderValue::from_str(v).unwrap(),
                );
            }

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&xtype).unwrap(),
            );
            let owner_id = service.user_service().username_to_uid(owner.clone()).await.unwrap();
            service.repo_service().sync_repo(owner.clone(), repo_name, owner_id).await.ok();
            response
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to open file"),
    }

}
