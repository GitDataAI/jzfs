use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::http::header;
use actix_web::web;
use actix_web::web::Payload;
use futures_util::StreamExt;

use crate::GIT_ROOT;
use crate::service::AppFsState;
use crate::transport::GitServiceType;
use crate::transport::Transport;

pub async fn http_pack(
    path : web::Path<(String, String)>,
    app : web::Data<AppFsState>,
    request : HttpRequest,
    transport : web::Data<Transport>,
    mut payload : Payload,
) -> impl actix_web::Responder {
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
    let service = if request.uri().to_string().contains("git-receive-pack") {
        GitServiceType::ReceivePack
    } else if request.uri().to_string().contains("git-upload-pack") {
        GitServiceType::UploadPack
    } else {
        return HttpResponse::BadRequest().body("Invalid service type");
    };

    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => bytes.extend_from_slice(&data),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to read request body: {}", e));
            }
        }
    }

    let mut respone = HttpResponseBuilder::new(StatusCode::OK);
    respone.append_header((
        "Content-Type",
        format!("application/x-git-{}-advertisement", service.to_string()),
    ));
    respone.append_header(("Connection", "Keep-Alive"));
    respone.append_header(("Transfer-Encoding", "chunked"));
    respone.append_header(("X-Content-Type-Options", "nosniff"));

    let body = bytes::Bytes::copy_from_slice(&bytes);
    let gzip = if request.headers().get(header::CONTENT_ENCODING).is_some() {
        if request.headers().get(header::CONTENT_ENCODING).unwrap() == "gzip" {
            true
        } else {
            false
        }
    } else {
        false
    };
    if let Ok(stream) = transport
        .pack(&path, service, Some(version), gzip, body, true)
        .await
    {
        respone.streaming(stream)
    } else {
        HttpResponse::InternalServerError().body("Failed to read request body")
    }
}
