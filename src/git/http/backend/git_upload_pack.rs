use crate::ROOT_PATH;
use crate::server::MetaData;
use actix_web::http::header::HeaderValue;
use actix_web::http::{StatusCode, header};
use actix_web::web::Payload;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder, web};
use async_graphql::async_stream;
use async_graphql::futures_util::StreamExt;
use flate2::read::GzDecoder;
use std::io;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub async fn git_upload_pack(
    http_request: HttpRequest,
    path: web::Path<(String, String)>,
    mut payload: Payload,
    service: web::Data<MetaData>,
) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo_name = repo_name.replace(".git", "");
    let repo_id = match service.repo_info(owner, repo_name).await {
        Ok(repo_id) => repo_id,
        Err(err) => return actix_web::HttpResponse::BadRequest().body(err.to_string()),
    };
    // TODO Token Authenticate
    let version = http_request
        .headers()
        .get("Git-Protocol")
        .unwrap_or(&HeaderValue::from_str("").unwrap())
        .to_str()
        .map(|s| s.to_string())
        .unwrap_or("".to_string());
    let mut resp = HttpResponseBuilder::new(StatusCode::OK);
    resp.append_header(("Content-Type", "application/x-git-upload-pack-advertise"));
    resp.append_header(("Connection", "Keep-Alive"));
    resp.append_header(("Transfer-Encoding", "chunked"));
    resp.append_header(("X-Content-Type-Options", "nosniff"));
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
    cmd.current_dir(PathBuf::from(ROOT_PATH).join(repo_id.uid.to_string()));
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
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to read request body: {}", e));
            }
        }
    }
    let body_data = match http_request
        .headers()
        .get(header::CONTENT_ENCODING)
        .and_then(|v| v.to_str().ok())
    {
        Some("gzip") => {
            let mut decoder = GzDecoder::new(Cursor::new(bytes));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to decode gzip body: {}", e));
            }
            decoded_data
        }
        _ => bytes.to_vec(),
    };
    if let Err(e) = stdin.write_all(&body_data) {
        return HttpResponse::InternalServerError()
            .body(format!("Failed to write to git process: {}", e));
    }
    drop(stdin);
    let body_stream = actix_web::body::BodyStream::new(async_stream::stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break,
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
    service._sync_repo(repo_id.uid).await.ok();
    resp.body(body_stream)
}
