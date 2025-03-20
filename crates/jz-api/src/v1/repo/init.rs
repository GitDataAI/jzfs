use crate::AppModule;
use crate::utils::request::RequestBody;
use crate::utils::session::from_session;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use jz_model::repository::RepositoryInitParam;
use serde_json::json;

pub async fn repo_init(
    session: Session,
    param: RequestBody<RepositoryInitParam>,
    module: Data<AppModule>,
) -> impl Responder {
    let opsuid = from_session(session).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": 401,
            "msg": "Unauthorized"
        }));
    }
    let opsuid = opsuid.unwrap();
    if let Err(e) = module.repo_init(opsuid, param.into_inner().inner).await {
        HttpResponse::Ok().json(json!({
            "code": 500,
            "msg": e.to_string()
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 200,
            "msg": "OK"
        }))
    }
}


pub async fn repo_access(
    session: Session,
    module: Data<AppModule>
) -> impl Responder {
    let opsuid = from_session(session).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": 401,
            "msg": "Unauthorized"
        }));
    }
    let opsuid = opsuid.unwrap();
    if let Ok(e) = module.repo_owners(opsuid).await {
        HttpResponse::Ok().json(json!({
            "code": 0,
            "data": e
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "OK"
        }))
    }
}