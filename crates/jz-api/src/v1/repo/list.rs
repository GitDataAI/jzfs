use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Path};
use serde_json::json;
use jz_module::AppModule;
use crate::utils::session::from_session;

pub async fn repo_user_list(
    session: Session,
    module: Data<AppModule>,
    owner: Path<String>
)
-> impl Responder {
    let opsuid = from_session(session).await.ok();
    match module.repo_list_info(opsuid, owner.into_inner()).await {
        Ok(e) => {
            HttpResponse::Ok().json(json!({
                "code": 0,
                "data": e
            }))
        },
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": e.to_string()
            }))
        }
    }
}