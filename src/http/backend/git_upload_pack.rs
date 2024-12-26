use std::io;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use actix_web::http::header::HeaderValue;
use actix_web::http::{header, StatusCode};
use actix_web::web::Payload;
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use crate::metadata::service::MetaService;
use crate::ROOT_PATH;

pub async fn git_upload_pack(http_request: HttpRequest, path: web::Path<(String, String)>, mut payload: Payload, service: web::Data<MetaService>) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name.clone()).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let owner_id = service.user_service().username_to_uid(owner.clone()).await.unwrap();
    service.repo_service().sync_repo(owner.clone(), repo_name, owner_id).await.ok();
    let version = http_request.headers().get("Git-Protocol").unwrap_or(&HeaderValue::from_str("").unwrap()).to_str().map(|s| s.to_string()).unwrap_or("".to_string());

    let mut resp = HttpResponseBuilder::new(StatusCode::OK);
    resp.append_header(("Content-Type", "application/x-git-upload-pack-advertise"));
    resp.append_header(("Connection", "Keep-Alive"));
    resp.append_header(("Transfer-Encoding","chunked"));
    resp.append_header(("X-Content-Type-Options","nosniff"));
    let mut cmd = Command::new("git");
    cmd.arg("upload-pack");
    cmd.arg("--stateless-rpc");
    cmd.arg(".");
    if !version.is_empty() {
        cmd.env("GIT_PROTOCOL", version.clone());
    }
    cmd.stderr(Stdio::piped());
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.current_dir(PathBuf::from(ROOT_PATH).join(repo_path));


    let span = cmd.spawn();
    let mut span = match span {
        Ok(span) => span,
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return HttpResponse::InternalServerError().body("Error running command");
        }
    };

    let mut stdin = span.stdin.take().unwrap();
    let mut stdout = span.stdout.take().unwrap();
    let _stderr = span.stderr.take().unwrap();
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => bytes.extend_from_slice(&data),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to read request body: {}", e)),
        }
    }
    let body_data = match http_request.headers().get(header::CONTENT_ENCODING).and_then(|v| v.to_str().ok()) {
        Some("gzip") => {
            let mut decoder = GzDecoder::new(Cursor::new(bytes));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return HttpResponse::InternalServerError().body(format!("Failed to decode gzip body: {}", e));
            }
            decoded_data
        },
        _ => bytes.to_vec(),
    };
    if let Err(e) = stdin.write_all(&body_data) {
        return HttpResponse::InternalServerError().body(format!("Failed to write to git process: {}", e));
    }
    drop(stdin);

    let body_stream = actix_web::body::BodyStream::new(async_stream::stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    yield Ok::<_, io::Error>(web::Bytes::copy_from_slice(&buffer[..n]))
                },
                Err(e) => {
                    eprintln!("Error reading stdout: {}", e);
                    break;
                }
            }
        }
    });
    resp.body(body_stream)
}