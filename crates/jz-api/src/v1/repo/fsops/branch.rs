use crate::utils::session::from_session;
use actix_session::Session;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, Responder};
use jz_module::AppModule;
use serde_json::json;

pub async fn list_branch(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String)>,
) -> impl Responder {
    let opsuid = from_session(session).await.ok();
    let (owner, repo) = path.into_inner();
    let result = module.repo_list_branch(opsuid, owner, repo).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(json!({
            "code": 0,
            "data": result
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": -1,
            "msg": err.to_string()
        })),
    }
}

pub async fn create_branch(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String)>,
    param: actix_web::web::Json<jz_module::repo::file::branch::RepoCreateBranch>,
) -> impl Responder {
    let opsuid = from_session(session).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": -1,
            "msg": "unauthorized"
        }));
    }
    let opsuid = opsuid.unwrap();
    let (owner, repo) = path.into_inner();
    let result = module
        .repo_create_branch(opsuid, owner, repo, param.into_inner())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "data": {}
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": -1,
            "msg": err.to_string()
        })),
    }
}

pub async fn delete_branch(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String, String)>,
) -> impl Responder {
    let opsuid = from_session(session).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": -1,
            "msg": "unauthorized"
        }));
    }
    let opsuid = opsuid.unwrap();
    let (owner, repo, name) = path.into_inner();
    let result = module.repo_delete_branch(opsuid, owner, repo, name).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "data": {}
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": -1,
            "msg": err.to_string()
        })),
    }
}

pub async fn rename_branch(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String, String, String)>,
) -> impl Responder {
    let opsuid = from_session(session).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": -1,
            "msg": "unauthorized"
        }));
    }
    let opsuid = opsuid.unwrap();
    let (owner, repo, name, new_name) = path.into_inner();
    let result = module
        .repo_rename_branch(opsuid, owner, repo, name, new_name)
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "data": {}
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": -1,
            "msg": err.to_string()
        })),
    }
}

pub async fn checkout_head(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String, String)>,
) -> impl Responder {
    let opsuid = from_session(session).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": -1,
            "msg": "unauthorized"
        }));
    }
    let opsuid = opsuid.unwrap();
    let (owner, repo, name) = path.into_inner();
    let result = module.repo_checkout_head(opsuid, owner, repo, name).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "data": {}
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": -1,
            "msg": err.to_string()
        })),
    }
}
