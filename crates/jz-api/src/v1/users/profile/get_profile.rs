use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use serde_json::json;
use jz_module::AppModule;
use crate::utils::session::from_session;

pub async fn get_profile(
    session: Session,
    module: Data<AppModule>,
) -> impl Responder {
    let opsuid = if let Ok(uid) = from_session(session).await {
        uid
    } else {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "no permission",
        }));
    };
    if let Ok(profile) = module.profile_info_by_id(opsuid).await {
        HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": "ok",
            "data": profile,
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "no permission",
        }))
    }
}