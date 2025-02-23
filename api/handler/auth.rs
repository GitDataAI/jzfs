use crate::api::write::AppWrite;
use crate::auth::captcha::CaptchaImage;
use crate::services::auth::apply::ApplyParma;
use crate::services::auth::passwd::AuthPasswd;
use crate::services::email::capctah::EmailCaptcha;
use crate::services::user::check::UserCheckParma;
use crate::services::AppState;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{web, HttpRequest, Responder};

pub async fn auth_passwd(
    session: Session,
    parma: Json<AuthPasswd>,
    request: HttpRequest,
    state: web::Data<AppState>
)
 -> impl Responder
{
    let captcha = match session.get::<String>("captcha") {
        Ok(Some(captcha)) => captcha,
        Ok(None) => return AppWrite::<()>::error("captcha error".to_string()),
        Err(_) => return AppWrite::<()>::error("captcha error".to_string()),
    };
    if let Some(captcha_parma) = request.headers().get("x-captcha") {
        let captcha_parma= match captcha_parma.to_str() {
            Ok(captcha_parma) => captcha_parma,
            Err(_) => return AppWrite::error("captcha error".to_string()),
        };
        if captcha.to_lowercase() != captcha_parma.to_lowercase(){
            return AppWrite::error("captcha error".to_string());
        }
    }
    match state.auth_passwd(parma.0).await {
        Ok(user) => {
            session.insert("user", serde_json::to_string(&user).unwrap()).ok();
            AppWrite::success("success".to_string())
        }
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn auth_apply(
    session: Session,
    parma: Json<ApplyParma>,
    request: HttpRequest,
    state: web::Data<AppState>
)
 -> impl Responder
{
    let captcha = match session.get::<String>("captcha") {
        Ok(Some(captcha)) => captcha,
        Ok(None) => return AppWrite::<()>::error("captcha error".to_string()),
        Err(_) => return AppWrite::<()>::error("cache error".to_string()),
    };
    if let Some(captcha_parma) = request.headers().get("x-captcha") {
        let captcha_parma= match captcha_parma.to_str() {
            Ok(captcha_parma) => captcha_parma,
            Err(_) => return AppWrite::error("captcha error".to_string()),
        };
        if captcha.to_lowercase() != captcha_parma.to_lowercase(){
            return AppWrite::error("captcha error".to_string());
        }
    }
    if let Ok(Some(x)) = session.get::<bool>("next") {
        if !x {
            return AppWrite::error("captcha error".to_string());
        }
    }
    match state.auth_apply(parma.0).await {
        Ok(user) => {
            session.insert("user", serde_json::to_string(&user).unwrap()).ok();
            AppWrite::success("success".to_string())
        }
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn auth_logout(
    session: Session,
)
 -> impl Responder
{
    session.renew();
    session.purge();
    AppWrite::<()>::success("success".to_string())
}

pub async fn auth_captcha(
    session: Session,
)
 -> impl Responder
{
    let captcha = CaptchaImage::new();
    session.insert("captcha", captcha.text).ok();
    captcha.base64
}


pub async fn auth_email_send(
    parma: Json<EmailCaptcha>,
    state: web::Data<AppState>,
    session: Session,
)
-> impl Responder
{
    match state.email_captcha(parma.0.email).await {
        Ok(captcha) => {
            session.insert("captcha", captcha.code).ok();
            AppWrite::<()>::success("success".to_string())
        },
        Err(err) => AppWrite::error(err.to_string()),
    }
}
pub async fn auth_email_check(
    session: Session,
    parma: Json<EmailCaptcha>,
)
-> impl Responder
{
    let captcha = match session.get::<String>("captcha") {
        Ok(Some(captcha)) => captcha,
        Ok(None) => return AppWrite::<()>::error("captcha error".to_string()),
        Err(_) => return AppWrite::<()>::error("cache error".to_string()),
    };
    if captcha != parma.0.code {
        return AppWrite::error("captcha error".to_string());
    }
    session.insert("captcha", "".to_string()).ok();
    session.insert("next", true).ok();
    AppWrite::<()>::success("success".to_string())
}

pub async fn auth_check(
    parma: Json<UserCheckParma>,
    state: web::Data<AppState>,
)
 -> impl Responder
{
    match state.users_check(parma.0).await {
        Ok(user) => {
            AppWrite::ok(user)
        },
        Err(err) => AppWrite::error(err.to_string()),
    }
}