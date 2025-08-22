use crate::AppStatus;
use actix_web::Responder;
use actix_web::web::Json;
use core::auth::user_register::AuthUserRegisterParam;
use core::email::captcha::EmailCaptchaSendParam;
use error::AppResult;
use session::Session;

pub async fn api_auth_user_register(
    session: Session,
    payload: Json<AuthUserRegisterParam>,
    core: AppStatus,
) -> impl Responder {
    core.auth_user_register(payload.clone(), session)
        .await
        .into_response()
}

pub async fn api_auth_user_register_after(
    payload: Json<AuthUserRegisterParam>,
    core: AppStatus,
) -> impl Responder {
    core.auth_user_register_before(payload.clone())
        .await
        .into_response()
}

pub async fn api_auth_user_register_after_captcha(
    session: Session,
    param: Json<EmailCaptchaSendParam>,
    core: AppStatus,
) -> impl Responder {
    core.email_captcha_send(param.clone(), session)
        .await
        .into_response()
}

pub async fn api_auth_user_register_after_captcha_verify(
    session: Session,
    payload: Json<EmailCaptchaSendParam>,
    core: AppStatus,
) -> impl Responder {
    core.email_captcha_verify(payload.clone(), session)
        .await
        .into_response()
}
