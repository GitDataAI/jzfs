use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use serde_json::json;
use uuid::Uuid;
use jz_module::AppModule;
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
                "msg": "Unauthorized",
            }))
        }
    };

    let tokens = match module.ssh_key_list(uid).await {
        Ok(tokens) => tokens,
        Err(e) => {
            return HttpResponse::Ok().json(json!({
                "msg": e.to_string(),
            }))
        }
    };

    HttpResponse::Ok().json(json!({
        "data": tokens,
        "msg": "ok",
    }))
}

pub async fn create_owner_ssh_token(
    session: Session,
    module: Data<AppModule>,
    param: actix_web::web::Json<jz_module::users::ssh_key::SshKeyParam>,
)
-> impl actix_web::Responder {

    let uid = match from_session(session).await {
        Ok(uid) => uid,
        Err(_) => {
            return HttpResponse::Ok().json(json!({
                "msg": "Unauthorized",
            }))
        }
    };
    match module.ssh_key_add(uid, param.into_inner()).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "msg": "ok",
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "msg": e.to_string(),
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
            }))
        }
    };
    match module.ssh_key_del(uid, path.into_inner()).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "msg": "ok",
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "msg": e.to_string(),
            }))
        }
    }
}