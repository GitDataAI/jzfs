use std::collections::HashMap;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use crate::metadata::service::MetaService;
use crate::ROOT_PATH;

pub async fn get_text_file(http_request: HttpRequest, path: web::Path<(String, String)>, service: web::Data<MetaService>) -> impl Responder{
    let (owner, repo_name) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name.clone()).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let path = http_request.uri().to_string().replace(&repo_path.clone(), "");

    let mut resp = HashMap::new();
    resp.insert("Pragma".to_string(),"no-cache".to_string());
    resp.insert("Cache-Control".to_string(),"no-cache, max-age=0, must-revalidate".to_string());
    resp.insert("Expires".to_string(),"Fri, 01 Jan 1980 00:00:00 GMT".to_string());

    let req_file = PathBuf::from(ROOT_PATH).join(repo_path).join(path);
    if !req_file.exists() {
        return HttpResponse::NotFound().body("File not found");
    }
    match NamedFile::open(req_file) {
        Ok(mut named_file) => {
            named_file = named_file.use_last_modified(true);
            let mut response = named_file.into_response(&http_request);
            for (k, v) in resp.iter() {
                response.headers_mut().insert(
                    k.to_string().parse().unwrap(),
                    HeaderValue::from_str(v).unwrap(),
                );
            }

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str("text/plain").unwrap(),
            );
            let owner_id = service.user_service().username_to_uid(owner.clone()).await.unwrap();
            service.repo_service().sync_repo(owner.clone(), repo_name, owner_id).await.ok();
            response
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to open file"),
    }
}
