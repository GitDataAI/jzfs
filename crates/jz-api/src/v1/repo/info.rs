use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path};
use serde_json::json;
use jz_module::AppModule;
use crate::utils::session::from_session;

pub async fn repo_info(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String,String)>,
) -> impl Responder {
    let opsuid = from_session(session).await;
    match module.repo_info_data(opsuid.ok(), path.0.clone(), path.1.clone()).await {
        Ok(e) => HttpResponse::Ok().json(json!({
            "code": 0,
            "data": e
        })),
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": e.to_string()
            }))
        }
    }
}


pub async fn repo_can_setting(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String,String)>,
) -> impl Responder {
    let opsuid = from_session(session).await;
    match module.repo_can_setting(opsuid.ok(), path.0.clone(), path.1.clone()).await {
        Ok(e) => HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": e
        })),
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": e.to_string()
            }))
        }
    }
}
