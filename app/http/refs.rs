use crate::app::http::{GitPack, GIT_ROOT};
use crate::app::services::AppState;
use poem::http::StatusCode;
use poem::web::{Data, Path};
use poem::{handler, IntoResponse, Request, Response};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tracing::info;


#[handler]
pub async fn refs(
    request: &Request,
    path: Path<(String, String)>,
    status: Data<&AppState>,
) -> impl IntoResponse {
    let version = request.headers().get("Git-Protocol").and_then(|x| x.to_str().ok());
    let mut response = Response::builder().status(StatusCode::OK);
    let url = request.uri().to_string().split("/")
        .map(|x| x.replace("/", ""))
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let mut cmd = Command::new("git");

    let server = if url.iter().any(|x| x.contains("git-upload-pack")) {
        response = response.header("Content-Type", "application/x-git-upload-pack-advertisement");
        cmd.arg("upload-pack");
        GitPack::UploadPack
    } else if url.iter().any(|x| x.contains("git-receive-pack")) {
        response = response.header("Content-Type", "application/x-git-receive-pack-advertisement");
        cmd.arg("receive-pack");
        GitPack::ReceivePack
    } else {
        return Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body("Protoc Not Support");
    };

    let (owner, repo) = path.0;
    let repo = repo.replace(".git", "");
    info!("repository ops: {}", format!("{}/{}", owner, repo));
    let repo = match status.repo_info(owner, repo).await {
        Ok(repo) => repo,
        Err(_) => return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Repo Not Found"),
    };

    let path = PathBuf::from(format!("{}/{}/{}/.git", GIT_ROOT, repo.node_uid, repo.uid));

    if !path.exists() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("repository not found");
    }

    cmd.arg("--stateless-rpc");
    cmd.arg("--advertise-refs");
    cmd.arg(".");
    cmd.current_dir(path);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    if let Some(version) = version {
        cmd.env("GIT_PROTOCOL", version);
    }

    let output = match cmd.output().await {
        Ok(output) => {
            info!("Command status: {:?}", output.status);
            output
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.to_string())
        }
    };

    let mut result = String::new();

    match server {
        GitPack::UploadPack => {
            result.push_str("001e# service=git-upload-pack\n");
            result.push_str("0000");
        }
        GitPack::ReceivePack => {
            result.push_str("001f# service=git-receive-pack\n");
            result.push_str("0000");
        }
    };

    result.push_str(std::str::from_utf8(&output.stdout).unwrap());
    response
        .header("Pragma", "no-cache")
        .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
        .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
        .body(result)
}
