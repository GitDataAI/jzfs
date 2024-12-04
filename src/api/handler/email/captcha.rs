use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::dto::email::{EmailCaptcha, EmailCaptchaCheck};
use crate::api::middleware::session::{ALLOW_NEXT_KEY, CAPTCHA};
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    post,
    tag = "email",
    path = "/api/v1/email/captcha",
    request_body(content = EmailCaptcha, content_type = "application/json"),
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "Captcha Expired"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_email_rand_captcha(
    session: Session, 
    service: web::Data<Service>,
    dto: web::Json<EmailCaptcha>
)
    -> impl Responder
{
        match service.email.generate_and_send_captcha(dto.email.clone()).await{
            Ok(result) => {
                session.insert(CAPTCHA, result).ok();
                R::<String>{
                    code: 200,
                    msg: Option::from("[Ok]".to_string()),
                    data: None,
                }
            }
            Err(e) => {
                R::<String>{
                    code: 400,
                    msg: Option::from(e.to_string()),
                    data: None,
                }
            }
        }
}

#[utoipa::path(
    post,
    tag = "email",
    path = "/api/v1/email/captcha/check",
    request_body(content = EmailCaptchaCheck, content_type = "application/json"),
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "Captcha Expired"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_email_captcha_check(
    session: Session, 
    dto: web::Json<EmailCaptchaCheck>
)
    -> impl Responder
{
    let captcha = session.get::<String>(CAPTCHA).unwrap();
    if captcha.is_none(){
        return R::<String>{
            code: 400,
            msg: Option::from("[Error] Captcha Expired".to_string()),
            data: None,
        }
    }
    if captcha.unwrap() == dto.code {
        session.insert(ALLOW_NEXT_KEY, true).ok();
        R::<String> {
            code: 200,
            msg: Option::from("[Ok]".to_string()),
            data: None,
        }
    } else {
        R::<String> {
            code: 400,
            msg: Option::from("[Error] Captcha Error".to_string()),
            data: None,
        }
    }
}