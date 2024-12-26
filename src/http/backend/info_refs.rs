use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use tracing::info;
use crate::metadata::service::MetaService;
use crate::ROOT_PATH;

pub async fn info_refs(req: HttpRequest, path: web::Path<(String, String)>, query: web::Query<HashMap<String, String>>, service: web::Data<MetaService>) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let service_name = query.get("service").unwrap_or(&"".to_string()).to_string();
    if service_name != "git-receive-pack" && service_name != "git-upload-pack" {
        return HttpResponse::BadRequest().body("Invalid service name");
    }
    let repo = service.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
    if repo.is_err() {
        return HttpResponse::NotFound().body("Not Found");
    }
    let repo_path = repo.unwrap().to_string();
    let service_name = service_name.replace("git-", "");
    let version = req.headers().get("Git-Protocol").unwrap_or(&HeaderValue::from_str("").unwrap()).to_str().map(|s| s.to_string()).unwrap_or("".to_string());
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
        Ok(output) =>{
            info!("Command status: {:?}", output.status);
            output
        },
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return HttpResponse::InternalServerError().body("Error running command");
        }
    };
    let mut resp = HttpResponseBuilder::new(StatusCode::OK);
    resp.append_header(("Content-Type", format!("application/x-git-{}-advertisement", service_name)));
    resp.append_header(("Pragma","no-cache"));
    resp.append_header(("Cache-Control","no-cache, max-age=0, must-revalidate"));
    resp.append_header(("Expires","Fri, 01 Jan 1980 00:00:00 GMT"));

    let mut body = String::new();

    match service_name.as_str() {
        "upload-pack" => {
            body.push_str(&"001e# service=git-upload-pack\n".to_string());
            body.push_str("0000");
        },
        "receive-pack" => {
            body.push_str(&"001f# service=git-receive-pack\n".to_string());
            body.push_str("0000");
        },
        _ => {}
    }
    body.push_str(&String::from_utf8(output.stdout).unwrap());
    resp.body(body.as_bytes().to_vec())
}