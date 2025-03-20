use crate::utils::request::RequestBody;
use crate::utils::session::from_session;
use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use jz_git::tree::GitTreeParam;
use jz_module::AppModule;
use serde_json::json;

pub async fn repo_tree(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String)>,
    payload: RequestBody<GitTreeParam>,
) -> impl actix_web::Responder {
    let (owner, repo) = path.into_inner();
    let opsuid = from_session(session).await.ok();
    let result = module
        .repo_tree(opsuid, owner, repo, payload.into_inner().inner)
        .await;

    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "error": err.to_string()
        })),
    }
}

pub async fn repo_tree_message(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String)>,
    payload: RequestBody<GitTreeParam>,
) -> impl actix_web::Responder {
    let (owner, repo) = path.into_inner();
    let opsuid = from_session(session).await.ok();
    let result = module
        .repo_tree_message(opsuid, owner, repo, payload.into_inner().inner)
        .await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::Ok().json(json!({
            "code": 500,
            "error": err.to_string()
        })),
    }
}
