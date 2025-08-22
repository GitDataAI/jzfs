use crate::GitContext;
use crate::service::GitServer;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path, Payload};
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use async_stream::stream;
use bytes::Bytes;
use futures_util::StreamExt;
use std::io;
use std::io::Read;
use std::process::{Command, Stdio};

pub async fn git_receive_pack(
    _request: HttpRequest,
    mut payload: Payload,
    path: Path<(String, String)>,
    status: Data<GitServer>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let repo = repo.replace(".git", "");
    let Ok(repo) = status.find_repo(&owner, &repo).await else {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND).body("Repository Not Found");
    };
    let Ok(path) =
        GitContext::try_from((repo.clone(), status.config.git.clone())).map(|x| x.path_dir)
    else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error");
    };
    if !path.exists() {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND).body("repository not found");
    }
    let mut child = match Command::new("git")
        .arg("receive-pack")
        .arg("--stateless-rpc")
        .arg(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        while let Some(Ok(bytes)) = payload.next().await {
            stdin.write_all(&bytes).ok();
        }
    }
    let mut stdout = child.stdout.unwrap();
    let body = actix_web::body::BodyStream::new(stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    yield Ok::<_, io::Error>(Bytes::copy_from_slice(&buffer[..n]));
                }
                Err(_e) => {
                    break;
                }
            }
        }
    });
    tokio::spawn(async move {
        status.sync_repo(repo.uid.clone()).await.ok();
    });
    HttpResponse::Ok()
        .content_type("application/x-git-receive-pack-result")
        .body(body)
}
