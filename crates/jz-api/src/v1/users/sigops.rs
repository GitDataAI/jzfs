use crate::utils::captcha::captcha_check_actix;
use crate::utils::request::RequestBody;
use crate::utils::session::to_session;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use jz_module::AppModule;
use jz_module::users::sig::{Sigin, SigupCheck};
use serde_json::json;

pub async fn sigin(
    session: Session,
    param: RequestBody<Sigin>,
    module: Data<AppModule>,
) -> impl Responder {
    if let Err(e) = captcha_check_actix(session.clone(), param.inner.captcha.clone()).await {
        return e;
    }
    match module.user_sigin(param.into_inner().inner).await {
        Ok(user) => {
            to_session(session, user.uid).await.unwrap();
            HttpResponse::Ok().json(json!({
                "code": 0,
                "data": {
                    "uid": user.uid,
                    "username": user.username,
                    "email": user.email,
                    "avatar": user.avatar,
                }
            }))
        }
        Err(e) => HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": e.to_string()
        })),
    }
}

pub async fn sigup(
    session: Session,
    param: RequestBody<jz_module::users::sig::Sigup>,
    module: Data<AppModule>,
) -> impl Responder {
    if let Err(e) = captcha_check_actix(session, param.inner.clone().captcha).await {
        return e;
    }
    match module.user_signup(param.into_inner().inner).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": "ok"
        })),
        Err(e) => HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": e.to_string()
        })),
    }
}

pub async fn sigout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().json(json!({
        "code": 0,
        "msg": "ok"
    }))
}

pub async fn sigin_check(session: Session, module: Data<AppModule>) -> impl Responder {
    match crate::utils::session::from_session(session).await {
        Ok(uid) => match module.user_info_by_id(uid).await {
            Ok(user) => HttpResponse::Ok().json(json!({
                "code": 0,
                "data": {
                    "uid": user.uid,
                    "username": user.username,
                    "email": user.email,
                    "avatar": user.avatar,
                }
            })),
            Err(_) => HttpResponse::Ok().json(json!({
                "code": 1,
                "msg": "用户不存在"
            })),
        },
        Err(_) => HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "未登录"
        })),
    }
}

pub async fn sigin_check_by_username(
    module: Data<AppModule>,
    payload: RequestBody<SigupCheck>
) -> impl Responder {
    match module.users_check(payload.into_inner().inner).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
        })),
        Err(e) => HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": e.to_string()
        })),
    }
}
