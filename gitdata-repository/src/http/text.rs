use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::Responder;
use actix_web::http::StatusCode;
use actix_web::http::header;
use actix_web::web;

use crate::GIT_ROOT;
use crate::service::AppFsState;
use crate::transport::Transport;

pub async fn http_text(
    path : web::Path<(String, String, String)>,
    app : web::Data<AppFsState>,
    request : HttpRequest,
    transport : web::Data<Transport>,
) -> impl Responder {
    let (owner, repo_name, file_path) = path.into_inner();
    let repo_name = repo_name.replace(".git", "");
    let repo = match app.info(owner, repo_name).await {
        Ok(repo) => repo,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };
    let node = repo.node;
    let repo_uid = repo.uid;
    let path = format!("{}/{}/{}", GIT_ROOT, node, repo_uid);

    let mut response = HttpResponseBuilder::new(StatusCode::OK);
    let url = request.uri().to_string();
    if url.ends_with(".pack") {
        response.insert_header((
            header::CONTENT_TYPE,
            "application/x-git-packed-objects".to_string(),
        ));
    } else if url.ends_with(".idx") {
        response.insert_header((
            header::CONTENT_TYPE,
            "application/x-git-packed-objects-toc".to_string(),
        ));
    } else {
        response.insert_header((
            header::CONTENT_TYPE,
            "application/x-git-loose-object".to_string(),
        ));
    }
    if let Ok(text) = transport.text(path, file_path).await {
        response.body(text)
    } else {
        HttpResponse::NotFound().body("Error")
    }
}
