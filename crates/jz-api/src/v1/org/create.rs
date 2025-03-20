use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use jz_module::AppModule;
use jz_module::org::create::OrgCreate;
use crate::utils::request::RequestBody;
use crate::utils::session::from_session;

pub async fn create_org(
    session: Session,
    app: web::Data<AppModule>,
    param: RequestBody<OrgCreate>,
)
-> impl Responder
{
    let opsuid = match from_session(session).await {
        Ok(opsuid) => opsuid,
        Err(err) => {
            return HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": err.to_string(),
            }))
        }
    };
    match app.create_org(opsuid, param.into_inner().inner).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "code": 0,
                "msg": "ok",
            }))
        },
        Err(err) => {
            HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": err.to_string(),
            }))
        }
    }
}