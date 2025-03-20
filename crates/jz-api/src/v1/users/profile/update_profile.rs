use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use serde_json::json;
use jz_module::AppModule;
use jz_module::users::profile::UpdateProfile;
use crate::utils::request::RequestBody;
use crate::utils::session::from_session;

pub async fn update_profile(
    session: Session,
    param: RequestBody<UpdateProfile>,
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
    if let Err(err) = module.profile_update(opsuid, param.into_inner().inner).await {
        HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": err.to_string(),
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": "ok",
        }))
    }
}
