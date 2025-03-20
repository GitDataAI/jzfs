use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use serde_json::json;
use jz_module::AppModule;
use crate::utils::request::RequestBody;

#[derive(Deserialize,Serialize)]
pub struct EmailCaptcha {
    pub email: String,
    pub code: String,
}


pub async fn email_captcha(
    session: Session,
    email: RequestBody<EmailCaptcha>,
    module: Data<AppModule>
) -> impl Responder {
    let email = email.inner.email.clone();
    if email.is_empty() {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "email is empty",
            "data": {},
        }));
    }
    if email.len() > 64 {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "email is too long",
            "data": {},
        }));
    }
    if !email.contains("@") {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "email is invalid",
            "data": {},
        }));
    }
    match module.send_email_captcha(email.clone()).await {
        Ok(x) => {
            session.insert("email", email).ok();
            session.insert("code", x).ok();
            HttpResponse::Ok().json(json!({
                "code": 0,
                "msg": "ok",
                "data": {},
            }))
        }
        Err(e) => {
            HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": e.to_string(),
                "data": {},
            }))
        }
    }
}

pub async fn email_captcha_check(
    session: Session,
    payload: RequestBody<EmailCaptcha>,
) -> impl Responder {
    let email = payload.inner.email.clone();
    let codes = payload.inner.code.clone();
    if email.is_empty() {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "email is empty",
            "data": {},
        }));
    }
    if email.len() > 64 {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "email is too long",
            "data": {},
        }));
    }
    if !email.contains("@") {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "email is invalid",
            "data": {},
        }));
    }
    let code = session.get::<String>("code").unwrap();
    if code.is_none() {
        return HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "code is empty",
            "data": {},
        }));
    }
    if code.unwrap() != codes.clone() {
        HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "code is invalid",
            "data": {},
        }))
    } else {
        session.insert("email", email).ok();
        session.remove("code");
        session.insert("allow", true).ok();
        HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": "ok",
            "data": {},
        }))
    }
}