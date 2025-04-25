use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path};
use serde_json::json;
use uuid::Uuid;
use jz_module::AppModule;

pub async fn list_context(session: Session, module: Data<AppModule>) -> impl Responder {
    let context = crate::utils::context::list_context(session).await;
    let mut result = vec![];
    for uid in context {
        let user = module.user_info_by_id(uid).await.unwrap();
        result.push(json!({
            "id": user.uid,
            "username": user.username,
            "avatar": user.avatar,
            "email": user.email,
        }));
    }
    HttpResponse::Ok().json(json!({
       "code": 0,
       "msg": "ok",
       "data": result
    }))
}

pub async fn current_context(session: Session) -> impl Responder {
    let uid = crate::utils::context::current_context(session).await;
    HttpResponse::Ok().json(json!({
        "code": 0,
        "msg": "ok",
        "data": uid
    }))
}

pub async fn switch_context(session: Session, uid: Path<Uuid>) -> impl Responder {
    crate::utils::context::switch_context(session, uid.into_inner()).await.unwrap();
    HttpResponse::Ok().json(json!({
        "code": 0,
        "msg": "ok",
    }))
}