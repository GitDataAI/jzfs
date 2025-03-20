use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use serde_json::json;
use jz_module::AppModule;
use crate::utils::session::from_session;


pub async fn repo_watch(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String)>,
) -> impl actix_web::Responder {
    let (owner, repo) = path.into_inner();
    match from_session(session).await {
        Ok(opsuid) => {
            if let Ok(_) = module.repo_watch(opsuid, owner, repo, 1).await {
                HttpResponse::Ok().json(json!({
                    "code": 0,
                    "msg": "Ok"
                }))
            } else {
                HttpResponse::Ok().json(json!({
                    "code": 1,
                    "msg": "OK"
                }))
            }
        },
        Err(_) => {
            HttpResponse::Ok().json(json!({
                "code": 401,
                "msg": "Unauthorized"
            }))
        }
    }
}
