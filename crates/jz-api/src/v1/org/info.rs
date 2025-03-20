use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use serde_json::json;
use jz_module::AppModule;
use crate::utils::session::from_session;

pub async fn org_info(
    org: Path<String>,
    app: Data<AppModule>
)
-> impl actix_web::Responder
{
    match app.org_by_name(org.into_inner()).await {
        Ok(org) => {
            HttpResponse::Ok().json(json!({
                "code": 0,
                "org": org,
                "msg": "ok",
            }))
        },
        Err(e) => {
            HttpResponse::NotFound().json(json!({
                "code": 1,
                "msg": e.to_string(),
            }))
        }
    }
}

pub async fn org_can_setting(
    session: actix_session::Session,
    orgs: Path<String>,
    app: Data<AppModule>
)
-> impl actix_web::Responder
{
    let opsuid = from_session(session).await;
    if opsuid.is_err() {
        return HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": false
        }));
    }
    let opsuid = opsuid.unwrap();
    let orgs = orgs.into_inner();
    match app.member_can_setting(orgs,opsuid).await {
        Ok(org) => {
            HttpResponse::Ok().json(json!({
                    "code": 0,
                    "msg": org,
                }))
        },
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": e.to_string(),
            }))
        }
    }
}
