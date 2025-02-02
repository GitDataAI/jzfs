use crate::ROOT_PATH;
use crate::server::MetaData;
use actix_files::NamedFile;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use std::collections::HashMap;
use std::path::PathBuf;

pub async fn objects_info_packs(
    http_request: HttpRequest,
    path: web::Path<(String, String)>,
    service: web::Data<MetaData>,
) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo_name = repo_name.replace(".git", "");
    let repo_id = match service.repo_info(owner, repo_name).await {
        Ok(repo_id) => repo_id,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };
    let repo_path = repo_id.uid.to_string();
    let path = "objects/info/packs".to_string();

    let mut map = HashMap::new();
    let time = chrono::Local::now();
    let expires = time + chrono::Duration::days(1);
    map.insert(
        "Date".to_string(),
        time.format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
    );
    map.insert(
        "Expires".to_string(),
        expires.format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
    );
    map.insert(
        "Cache-Control".to_string(),
        "public, max-age=86400".to_string(),
    );
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
                HeaderValue::from_str("application/x-git-loose-object").unwrap(),
            );
            service._sync_repo(repo_id.uid).await.ok();
            response
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to open file"),
    }
}
