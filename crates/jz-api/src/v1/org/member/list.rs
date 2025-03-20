use actix_session::Session;
use actix_web::{web, HttpResponse};
use actix_web::web::Path;
use serde_json::json;
use jz_module::AppModule;
use crate::utils::session::from_session;

pub async fn org_user_list(
    user: Path<String>,
    app: web::Data<AppModule>,
)
-> impl actix_web::Responder
{
    let users = match app.user_info_by_username(user.into_inner()).await {
        Ok(user)=>user,
        Err(_)=>return HttpResponse::Ok()
            .json(json!({
                "code": 1,
                "msg": "User Not Found #1"
            }))
    };
    let orgs = match app.member_orgs(users.uid).await {
        Ok(orgs)=>orgs,
        Err(_)=>return HttpResponse::Ok()
            .json(json!({
                "code": 1,
                "msg": "User Not Found #2"
            }))
    };
    HttpResponse::Ok().json(json!({
        "code": 0,
        "msg": "ok",
        "data": orgs
    }))
}

pub async fn org_owner_list(
    session: Session,
    app: web::Data<AppModule>,
)
-> impl actix_web::Responder
{
    let users = match from_session(session).await {
        Ok(user)=>user,
        Err(_)=>return HttpResponse::Ok()
            .json(json!({
                "code": 1,
                "msg": "User Not Found #1"
            }))
    };
    let orgs = match app.member_owner_access(users).await {
        Ok(orgs)=>orgs,
        Err(_)=>return HttpResponse::Ok()
            .json(json!({
                "code": 1,
                "msg": "User Not Found #2"
            }))
    };
    HttpResponse::Ok().json(json!({
        "code": 0,
        "msg": "ok",
        "data": orgs
    }))
}


pub async fn org_member_list(
    org: Path<String>,
    app: web::Data<AppModule>,
)
-> impl actix_web::Responder
{
    let orgs = match app.member_list_by_name_values(org.into_inner()).await {
        Ok(orgs)=>orgs,
        Err(_)=>return HttpResponse::Ok()
            .json(json!({
                "code": 1,
                "msg": "Org Not Found"
            }))
    };
    HttpResponse::Ok().json(json!({
        "code": 0,
        "msg": "ok",
        "data": orgs
    }))
}
