use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::web::Data;
use serde_json::json;
use jz_module::AppModule;
use crate::utils::session::from_session;

pub async fn list_owner_token(
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
    match module.token_list(uid).await {
        Ok(tokens) => {
            HttpResponse::Ok().json(json!({
                "tokens": tokens,
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "msg": e.to_string()
            }))
        }
    }
}

pub async fn create_owner_token(
    session: Session,
    module: Data<AppModule>,
    param: actix_web::web::Json<jz_module::users::token::TokenCreate>,
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
    match module.token_create(uid, param.into_inner()).await {
        Ok(token) => {
            HttpResponse::Ok().json(json!({
                "token": token,
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "msg": e.to_string()
            }))
        }
    }
}

pub async fn delete_owner_token(
    session: Session,
    module: Data<AppModule>,
    param: actix_web::web::Json<jz_module::users::token::TokenDelete>,
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
    match module.token_delete(uid, param.into_inner()).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "msg": e.to_string()
            }))
        }
    }
}