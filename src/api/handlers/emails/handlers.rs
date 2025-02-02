use crate::api::app_writer::AppWrite;
use crate::api::handlers::emails::options::{EmailCaptcha, EmailCaptchaCheck};
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{Responder, web};

pub async fn email_captcha(
    session: Session,
    email: web::Json<EmailCaptcha>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let email = email.into_inner();
    match meta.email_captcha(&email.email, session).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
pub async fn email_captcha_check(
    session: Session,
    captcha: web::Json<EmailCaptchaCheck>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let captcha = captcha.into_inner();
    match meta.email_captcha_check(&captcha.code, session).await {
        Ok(_) => AppWrite::<Option<String>>::ok(None),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
