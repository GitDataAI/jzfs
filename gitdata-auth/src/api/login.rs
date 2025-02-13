use actix_session::Session;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::web;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::write::AppWrite;

use crate::server::AppAuthState;
use crate::server::passwd::PasswordAuth;

/*
 * 登录
 * @param {PasswordAuth} param
 * @param {Session} session
 * @header {x-captcha}
 * @header {x-fingerprint}
 * @return {UsersSessionModel} param
 * @Author: ZhenYi
 */
pub async fn auth_password(
    state : web::Data<AppAuthState>,
    param : web::Json<PasswordAuth>,
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
    }
    let param = param.into_inner();
    match state.auth_password(param).await {
        Ok(model) => {
            session.insert(USER_SESSION_KEY.to_string(), &model).ok();
            AppWrite::ok(model)
        }
        Err(err) => AppWrite::error(err.to_string()),
    }
}
