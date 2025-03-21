use actix_session::Session;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, Responder};
use jz_module::AppModule;
use serde_json::json;
use uuid::Uuid;

pub async fn switch_context(session: Session, param: Path<Uuid>) -> impl Responder {
    match crate::utils::context::switch_context(session, param.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": "ok"
        })),
        Err(_) => HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "未登录"
        })),
    }
}

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