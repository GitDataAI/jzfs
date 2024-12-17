use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::email_dto::{EmailCaptcha, EmailCaptchaCheck};
use crate::metadata::service::MetaService;
pub const ALLOW_NEXT_KEY: &str = "allow_next";
pub const CAPTCHA: &str = "captcha";

#[utoipa::path(
    post,
    tag = "email",
    path = "/api/v1/email/captcha",
    request_body = EmailCaptcha,
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]
pub async fn api_email_rand_captcha(
    session: Session,
    service: web::Data<MetaService>,
    dto: web::Json<EmailCaptcha>
)
    -> impl Responder
{
    match service.email_service().generate_and_send_captcha(dto.email.clone()).await{
        Ok(result) => {
            session.insert(CAPTCHA, result).ok();
            AppWrite::<String>::ok_msg("[Ok]".to_string())
        }
        Err(e) => {
           AppWrite::error(e.to_string())
        }
    }
}

#[utoipa::path(
    put,
    tag = "email",
    path = "/api/v1/email/captcha",
    request_body = EmailCaptchaCheck,
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]
pub async fn api_email_captcha_check(
    session: Session,
    dto: web::Json<EmailCaptchaCheck>
)
    -> impl Responder
{
    let captcha = session.get::<String>(CAPTCHA).unwrap();
    if captcha.is_none(){
        return AppWrite::<String>::error("[Error] Captcha Expired".to_string())
    }
    if captcha.unwrap() == dto.code {
        session.insert(ALLOW_NEXT_KEY, true).ok();
        AppWrite::ok_msg("[Ok]".to_string())
    } else {
        AppWrite::error("[Error] Captcha Error".to_string())
    }
}