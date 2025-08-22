use crate::GitContext;
use crate::service::GitServer;
use crate::transport::GitPack;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponseBuilder, Responder};
use std::process::Stdio;
use tokio::process::Command;
use tracing::info;

pub async fn git_refs(
    request: HttpRequest,
    path: Path<(String, String)>,
    status: Data<GitServer>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let version = request
        .headers()
        .get("Git-Protocol")
        .and_then(|x| x.to_str().ok());
    let mut response = HttpResponseBuilder::new(StatusCode::OK);
    response
        .insert_header(("Pragma", "no-cache"))
        .insert_header(("Cache-Control", "no-cache, max-age=0, must-revalidate"))
        .insert_header(("Expires", "Fri, 01 Jan 1980 00:00:00 GMT"));
    let url = request
        .uri()
        .to_string()
        .split("/")
        .map(|x| x.replace("/", ""))
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let mut cmd = Command::new("git");
    info!("request url: {}", url.join("/"));
    let server = if url.iter().any(|x| x.contains("git-upload-pack")) {
        response.insert_header((
            "Content-Type",
            "application/x-git-upload-pack-advertisement",
        ));
        cmd.arg("upload-pack");
        GitPack::UploadPack
    } else if url.iter().any(|x| x.contains("git-receive-pack")) {
        response.insert_header((
            "Content-Type",
            "application/x-git-receive-pack-advertisement",
        ));
        cmd.arg("receive-pack");

        GitPack::ReceivePack
    } else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("Protoc Not Support");
    };

    let repo = repo.replace(".git", "");
    let Ok(repo) = status.find_repo(&owner, &repo).await else {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND).body("Repository Not Found");
    };
    let Ok(path) = GitContext::try_from((repo, status.config.git.clone())).map(|x| x.path_dir)
    else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error");
    };
    if !path.exists() {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND).body("repository not found");
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
            return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body(e.to_string());
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
    response.body(result)
}
