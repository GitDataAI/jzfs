use crate::ROOT_PATH;
use crate::server::MetaData;
use actix_web::http::StatusCode;
use actix_web::http::header::HeaderValue;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder, web};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tracing::info;

pub async fn info_refs(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    query: web::Query<HashMap<String, String>>,
    service: web::Data<MetaData>,
) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo_name = repo_name.replace(".git", "");
    let repo_id = match service.repo_info(owner, repo_name).await {
        Ok(repo_id) => repo_id,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };
    let repo_path = repo_id.uid.to_string();
    let service_name = query.get("service").unwrap_or(&"".to_string()).to_string();
    if service_name != "git-receive-pack" && service_name != "git-upload-pack" {
        return HttpResponse::BadRequest().body("Invalid service name");
    }

    let service_name = service_name.replace("git-", "");
    let version = req
        .headers()
        .get("Git-Protocol")
        .unwrap_or(&HeaderValue::from_str("").unwrap())
        .to_str()
        .map(|s| s.to_string())
        .unwrap_or("".to_string());
    let mut cmd = Command::new("git");
    cmd.arg(service_name.clone());
    cmd.arg("--stateless-rpc");
    cmd.arg("--advertise-refs");
    cmd.arg(".");
    cmd.current_dir(PathBuf::from(ROOT_PATH).join(repo_path.clone()));
    if !version.is_empty() {
        cmd.env("GIT_PROTOCOL", version.clone());
    }

    let output = match cmd.output() {
        Ok(output) => {
            info!("Command status: {:?}", output.status);
            output
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return HttpResponse::InternalServerError().body("Error running command");
        }
    };
    let mut resp = HttpResponseBuilder::new(StatusCode::OK);
    resp.append_header((
        "Content-Type",
        format!("application/x-git-{}-advertisement", service_name),
    ));
    resp.append_header(("Pragma", "no-cache"));
    resp.append_header(("Cache-Control", "no-cache, max-age=0, must-revalidate"));
    resp.append_header(("Expires", "Fri, 01 Jan 1980 00:00:00 GMT"));

    let mut body = String::new();

    match service_name.as_str() {
        "upload-pack" => {
            body.push_str(&"001e# service=git-upload-pack\n".to_string());
            body.push_str("0000");
        }
        "receive-pack" => {
            body.push_str(&"001f# service=git-receive-pack\n".to_string());
            body.push_str("0000");
        }
        _ => {}
    }
    service._sync_repo(repo_id.uid).await.ok();
    body.push_str(&String::from_utf8(output.stdout).unwrap());
    resp.body(body.as_bytes().to_vec())
}
