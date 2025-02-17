use std::io::Write;
use std::io::Read;
use crate::app::http::{GitPack, GIT_ROOT};
use crate::app::services::AppState;
use flate2::bufread::GzDecoder;
use poem::http::StatusCode;
use poem::web::{Data, Path};
use poem::{handler, Body, IntoResponse, Request, Response};
use std::io;
use std::io::Cursor;
use std::process::Stdio;
use bytes::Bytes;
use std::process::Command;
use async_stream::stream;
use tracing::{error, info};

#[handler]
pub async fn pack(
    request: &Request,
    payload: Bytes,
    path: Path<(String, String)>,
    status: Data<&AppState>,
) -> impl IntoResponse {
    let mut bytes = if let Some(zip) = request.header("content-encoding") {
        if zip == "gzip" {
            let mut decoder = GzDecoder::new(Cursor::new(payload.clone()));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(e.to_string());
            }
            decoded_data
        } else {
             payload.to_vec()
        }
    }else {
         payload.to_vec()
    };

    let version = request.headers().get("Git-Protocol").and_then(|x| x.to_str().ok());

    let mut response = Response::builder()
        .status(StatusCode::OK);
    let url = request.uri().path().split("/")
        .map(|x| x.replace("/", ""))
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let mut cmd = Command::new("git");

    let _server = if url.iter().any(|x| x.contains("git-upload-pack")) {
        response = response.header("Content-Type", "application/x-git-upload-pack-result");
        cmd.arg("upload-pack");
        GitPack::UploadPack
    } else if url.iter().any(|x| x.contains("git-receive-pack")) {
        response = response.header("Content-Type", "application/x-git-receive-pack-result");
        cmd.arg("receive-pack");
        GitPack::ReceivePack
    } else {
        return Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body("Protoc Not Support");
    };

    let (owner, repo) = path.0;
    let repo = repo.replace(".git", "");
    let repo = match status.repo_info(owner, repo).await {
        Ok(repo) => repo,
        Err(_) => return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Repo Not Found"),
    };

    cmd.arg("--stateless-rpc");
    cmd.arg(".");
    let path = format!("{}/{}/{}/.git", GIT_ROOT, repo.node_uid, repo.uid);
    cmd.current_dir(path);
    if let Some(version) = version {
        cmd.env("GIT_PROTOCOL", version);
    }
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.to_string());
        }
    };

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let _stderr = child.stderr.take().unwrap();

    if let Err(e) = stdin.write_all(&bytes) {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string());
    }
    drop(stdin);

    let body = Body::from_bytes_stream(stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    yield Ok::<_, io::Error>(Bytes::copy_from_slice(&buffer[..n]));
                }
                Err(e) => {
                    error!("Error reading stdout: {}", e);
                    break;
                }
            }
        }
    });
    let status = status.clone();
    tokio::spawn(async move {
        match status.repo_sync(repo.uid).await{
            Ok(_) => {
                info!("Repo sync success");
            }
            Err(e) => {
                error!("Error syncing repo: {}", e);
            }
        }
    });


    response
        .header("Pragma", "no-cache")
        .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
        .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
        .header("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload")
        .header("X-Frame-Options", "DENY")
        .body(body)
}
