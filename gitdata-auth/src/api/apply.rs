use actix_session::Session;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::web;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::write::AppWrite;

use crate::server::AppAuthState;
use crate::server::ctrl::UsersApply;

pub async fn auth_apply(
    state : web::Data<AppAuthState>,
    param : web::Json<UsersApply>,
    session : Session,
    request : HttpRequest,
) -> impl Responder {
    let captcha = match session.get::<String>("captcha") {
        Ok(captcha) => match captcha {
            Some(captcha) => captcha,
            None => return AppWrite::error("captcha error".to_string()),
        },
        Err(_) => return AppWrite::error("captcha error".to_string()),
    };
    let fingerprint = match session.get::<String>("fingerprint") {
        Ok(fingerprint) => match fingerprint {
            Some(fingerprint) => fingerprint,
            None => return AppWrite::error("fingerprint error".to_string()),
        },
        Err(_) => return AppWrite::error("fingerprint error".to_string()),
    };
    if let Some(x_fingerprint) = request.headers().get("x-fingerprint") {
        if fingerprint != x_fingerprint.to_str().unwrap() {
            return AppWrite::error("fingerprint error".to_string());
        }
    }
    if let Some(x_captcha) = request.headers().get("x-captcha") {
        if captcha != x_captcha.to_str().unwrap() {
            return AppWrite::error("captcha error".to_string());
        }
    }
    if let Some(fingerprint) = request.headers().get("x-fingerprint") {
        if let Some(captcha) = session.get::<String>("captcha").unwrap() {
            if fingerprint.to_str().unwrap() != captcha {
                return AppWrite::error("fingerprint error".to_string());
            }
        }
    };
    if let Some(next) = session.get::<bool>("next").unwrap_or(Some(false)) {
        if !next {
            return AppWrite::error("next error".to_string());
        }
    } else {
        return AppWrite::error("next error".to_string());
    }
    match state.auth_apply(param.into_inner()).await {
        Ok(user) => {
            session.insert("next", false).ok();
            session.insert(USER_SESSION_KEY, user.clone()).ok();
            AppWrite::ok(user)
        }
        Err(err) => AppWrite::error(err.to_string()),
    }
}
