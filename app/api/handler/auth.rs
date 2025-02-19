use crate::app::api::write::AppWrite;
use crate::app::auth::captcha::CaptchaImage;
use crate::app::services::auth::apply::ApplyParma;
use crate::app::services::auth::passwd::AuthPasswd;
use crate::app::services::email::capctah::EmailCaptcha;
use crate::app::services::user::check::UserCheckParma;
use crate::app::services::AppState;
use poem::session::Session;
use poem::web::Json;
use poem::{handler, web, IntoResponse, Request};

#[handler]
pub async fn auth_passwd(
    session: &Session,
    parma: Json<AuthPasswd>,
    request: &Request,
    state: web::Data<&AppState>
)
 -> impl IntoResponse
{
    let captcha = match session.get::<String>("captcha") {
        Some(captcha) => captcha,
        None => return AppWrite::<()>::error("captcha error".to_string()),
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
            session.set("user", serde_json::to_string(&user).unwrap());
            AppWrite::success("success".to_string())
        }
        Err(err) => AppWrite::error(err.to_string()),
    }
}

#[handler]
pub async fn auth_apply(
    session: &Session,
    parma: Json<ApplyParma>,
    request: &Request,
    state: web::Data<&AppState>
)
 -> impl IntoResponse
{
    let captcha = match session.get::<String>("captcha") {
        Some(captcha) => captcha,
        None => return AppWrite::<()>::error("captcha error".to_string()),
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
    if let Some(x) = session.get::<bool>("next") {
        if !x {
            return AppWrite::error("captcha error".to_string());
        }
    }
    match state.auth_apply(parma.0).await {
        Ok(user) => {
            session.set("user", serde_json::to_string(&user).unwrap());
            AppWrite::success("success".to_string())
        }
        Err(err) => AppWrite::error(err.to_string()),
    }
}

#[handler]
pub async fn auth_logout(
    session: &Session,
)
 -> impl IntoResponse
{
    session.renew();
    session.purge();
    AppWrite::<()>::success("success".to_string())
}

#[handler]
pub async fn auth_captcha(
    session: &Session,
)
 -> impl IntoResponse
{
    let captcha = CaptchaImage::new();
    session.set("captcha", captcha.text);
    captcha.base64
}


#[handler]
pub async fn auth_email_send(
    parma: Json<EmailCaptcha>,
    state: web::Data<&AppState>,
    session: &Session,
) 
-> impl IntoResponse
{
    match state.email_captcha(parma.0.email).await {
        Ok(captcha) => {
            session.set("captcha", captcha.code);
            AppWrite::<()>::success("success".to_string())
        },
        Err(err) => AppWrite::error(err.to_string()),
    }
}
#[handler]
pub async fn auth_email_check(
    session: &Session,
    parma: Json<EmailCaptcha>,
) 
-> impl IntoResponse
{
    let captcha = match session.get::<String>("captcha") {
        Some(captcha) => captcha,
        None => return AppWrite::<()>::error("captcha error".to_string()),
    };
    if captcha != parma.0.code {
        return AppWrite::error("captcha error".to_string());
    }
    session.set("captcha", "".to_string());
    session.set("next", true);
    AppWrite::<()>::success("success".to_string())
}

#[handler]
pub async fn auth_check(
    parma: Json<UserCheckParma>,
    state: web::Data<&AppState>,
)
 -> impl IntoResponse
{
    match state.users_check(parma.0).await {
        Ok(user) => {
            AppWrite::ok(user)
        },
        Err(err) => AppWrite::error(err.to_string()),
    }
}