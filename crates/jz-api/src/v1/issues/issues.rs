use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use serde_json::json;
use jz_module::AppModule;
use crate::utils::request::RequestBody;
use crate::utils::session::from_session;

pub async fn issues_create(
    session: Session,
    module: Data<AppModule>,
    path: Path<(String, String)>,
    param: RequestBody<jz_module::issue::issues::AddIssues>,
)
-> impl actix_web::Responder {
    let opsuid = match from_session(session).await {
        Ok(uid) => uid,
        Err(_) => return HttpResponse::Ok().json(json!({
            "msg": "unauthorized",
            "code": 401,
        })),
    };
    match module.issue_add(path.0.clone(), path.1.clone(), param.into_inner().inner, opsuid).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "msg": "success",
            "code": 200,
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "msg": err.to_string(),
            "code": 500,
        })),
    }
}

