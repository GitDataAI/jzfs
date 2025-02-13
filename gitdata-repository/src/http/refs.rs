use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::Responder;
use actix_web::http::StatusCode;
use actix_web::web;

use crate::GIT_ROOT;
use crate::service::AppFsState;
use crate::transport::GitServiceType;
use crate::transport::Transport;

pub async fn http_refs(
    path : web::Path<(String, String)>,
    app : web::Data<AppFsState>,
    request : HttpRequest,
    transport : web::Data<Transport>,
) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo_name = repo_name.replace(".git", "");
    let repo = match app.info(owner, repo_name).await {
        Ok(repo) => repo,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };
    let node = repo.node;
    let repo_uid = repo.uid;
    let path = format!("{}/{}/{}", GIT_ROOT, node, repo_uid);

    let version = request
        .headers()
        .get("Git-Protocol")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let service = if request
        .query_string()
        .to_string()
        .contains("git-receive-pack")
    {
        GitServiceType::ReceivePack
    } else if request
        .query_string()
        .to_string()
        .contains("git-upload-pack")
    {
        GitServiceType::UploadPack
    } else {
        return HttpResponse::BadRequest().body("Invalid service type");
    };
    let result = transport.refs(&path, service.clone(), Some(version)).await;

    let mut respone = HttpResponseBuilder::new(StatusCode::OK);
    respone.append_header((
        "Content-Type",
        format!("application/x-git-{}-advertisement", service.to_string()),
    ));
    respone.append_header(("Pragma", "no-cache"));
    respone.append_header(("Cache-Control", "no-cache, max-age=0, must-revalidate"));
    respone.append_header(("Expires", "Fri, 01 Jan 1980 00:00:00 GMT"));
    match result {
        Ok(mut result) => {
            match service {
                GitServiceType::ReceivePack => {
                    result.push_str(&"001f# service=git-receive-pack\n".to_string());
                    result.push_str("0000");
                }
                GitServiceType::UploadPack => {
                    result.push_str(&"001e# service=git-upload-pack\n".to_string());
                    result.push_str("0000");
                }
                _ => {}
            };
            respone.body(result)
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
