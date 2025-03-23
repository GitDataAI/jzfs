use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use serde_json::json;
use uuid::Uuid;
use jz_module::AppModule;
use crate::utils::request::RequestBody;
use crate::utils::session::from_session;

pub async fn list_owner_ssh_key(
    session: Session,
    module: Data<AppModule>,
)
-> impl actix_web::Responder {

    let uid = match from_session(session).await {
        Ok(uid) => uid,
        Err(_) => {
            return HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": "Unauthorized",
            }))
        }
    };

    let tokens = match module.ssh_key_list(uid).await {
        Ok(tokens) => tokens,
        Err(e) => {
            return HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": e.to_string(),
            }))
        }
    };

    HttpResponse::Ok().json(json!({
        "code": 0,
        "data": tokens,
        "msg": "ok",
    }))
}

pub async fn create_owner_ssh_token(
    session: Session,
    module: Data<AppModule>,
    param: RequestBody<jz_module::users::ssh_key::SshKeyParam>,
)
-> impl actix_web::Responder {

    let uid = match from_session(session).await {
        Ok(uid) => uid,
        Err(_) => {
            return HttpResponse::Ok().json(json!({
                "msg": "Unauthorized",
                "code": 1,
            }))
        }
    };
    match module.ssh_key_add(uid, param.into_inner().inner).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "msg": "ok",
                "code": 0,
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "msg": e.to_string(),
                "code": 1,
            }))
        }
    }
}

pub async fn delete_owner_ssh_token(
    session: Session,
    module: Data<AppModule>,
    path: Path<Uuid>,
)
-> impl actix_web::Responder {
    let uid = match from_session(session).await {
        Ok(uid) => uid,
        Err(_) => {
            return HttpResponse::Ok().json(json!({
                "msg": "Unauthorized",
                "code": 1,
            }))
        }
    };
    match module.ssh_key_del(uid, path.into_inner()).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "msg": "ok",
                "code": 0,
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "msg": e.to_string(),
                "code": 1,
            }))
        }
    }
}