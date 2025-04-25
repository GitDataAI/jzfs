use actix_session::Session;
use actix_web::web;
use actix_web::web::Data;
use serde_json::json;
use jz_service::app::AppService;
use jz_service::users::auth::{Signin, Signup};
use crate::utils::captcha::captcha_check;
use crate::utils::request::RequestContext;

pub async fn auth_signup(
    session: Session,
    service: Data<AppService>,
    param: RequestContext<Signup>,
) -> web::Json<serde_json::Value> {
    let param = param.inner;
    match captcha_check(session.clone(), param.captcha.clone()) {
        Ok(false) => {
            return web::Json(json!({
                "code": 1,
                "msg": "captcha error",
                "data": {}
            }))
        }
        _ => {}
    }
    let user = match service.user_auth_signup(param).await {
        Ok(user) => user,
        Err(err) => {
            return web::Json(json!({
                "code": 1,
                "msg": err.to_string(),
                "data": {}
            }))
        }
    };
    match session.insert("current_uid", user.uid) {
        Ok(_) => {}
        Err(err) => {
            return web::Json(json!({
                "code": 1,
                "msg": err.to_string(),
                "data": {}
            }))
        }
    }
    web::Json(json!({
        "code": 0,
        "msg": "ok",
        "data": user.uid
    }))
}

pub async fn auth_signin(
    session: Session,
    service: Data<AppService>,
    param: RequestContext<Signin>,
) -> web::Json<serde_json::Value> {
    let param = param.inner;
    match captcha_check(session.clone(), param.captcha.clone()) {
        Ok(false) => {
            return web::Json(json!({
                "code": 1,
                "msg": "captcha error",
                "data": {}
            }))
        }
        _ => {}
    }
    let user = match service.user_auth_signin(param).await {
        Ok(user) => user,
        Err(err) => {
            return web::Json(json!({
                "code": 1,
                "msg": err.to_string(),
                "data": {}
            }))
        }
    };
    match session.insert("current_uid", user.uid) {
        Ok(_) => {}
        Err(err) => {
            return web::Json(json!({
                "code": 1,
                "msg": err.to_string(),
                "data": {}
            }))
        }
    }
    web::Json(json!({
        "code": 0,
        "msg": "ok",
        "data": user.uid
    }))
}