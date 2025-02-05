use actix_web::{web, HttpRequest, Responder};
use captcha_rs::CaptchaBuilder;
use serde::Deserialize;
use serde_json::json;
use sha256::Sha256Digest;
use actix_session::Session;
use lib_entity::write::AppWrite;
use lib_mq::client::client::AppKafkaClient;
use lib_mq::EMAIL_TOPIC;
use lib_mq::server::email::captcha::EmailCaptcha;
use lib_mq::server::email::{EmailEvent, EmailType};
/*
 * 登录验证码
 * @param {Session} session
 * @header {x-fingerprint}
 * @return { image: base64, fingerprint: text } 
 * @Author: ZhenYi
 */
pub async fn auth_captcha_image(
    session: Session,
    request: HttpRequest,
)
    -> impl Responder
{
    let header = request.headers()
        .iter()
        .map(|(k, v)| format!("{}-{}", k.to_string(), v.len()))
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let fingerprint = header.digest();
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .complexity(1)
        .compression(40)
        .build();
    session.insert("fingerprint", &fingerprint).ok();
    session.insert("captcha", &captcha.text).ok();
    let base64 = captcha.to_base64();
    AppWrite::ok(json!(
        {
            "image": base64,
            "fingerprint": fingerprint
        }
    ))
}


#[derive(Deserialize)]
pub struct EmailCaptchaSend {
    email: String,
}
#[derive(Deserialize)]
pub struct EmailCaptchaCheck {
    email: String,
    code: String,
}


pub async fn auth_captcha_email_send(
    session: Session,
    payload: web::Json<EmailCaptchaSend>,
    mq: web::Data<AppKafkaClient>
)
    -> impl Responder
{
    let email = payload.email.clone();
    let captcha = EmailCaptcha::generate_captcha(email.clone());
    let email_event = EmailType::Captcha(captcha.clone());
    let data = match serde_json::to_vec(&email_event){
        Ok(payload) => payload,
        Err(_) => return AppWrite::error("序列化失败".to_string())
    };
    match mq.send(EMAIL_TOPIC.to_string(), None, data).await {
        Ok(_) => {
            session.insert(format!("captcha:{}",email), captcha.code).ok();
            AppWrite::ok("发送成功".to_string())
        },
        Err(_) => AppWrite::error("发送失败".to_string())
    }
}


pub async fn auth_captcha_email_check(
    session: Session,
    payload: web::Json<EmailCaptchaCheck>,
)
    -> impl Responder
{
    let email = payload.email.clone();
    let code = payload.code.clone();
    let captcha = match session.get::<String>(&format!("captcha:{}",email)) {
        Ok(captcha) => captcha,
        Err(_) => return AppWrite::error("验证失败".to_string())
    };
    match captcha {
        Some(captcha) => {
            if captcha == code {
                session.insert("next",true).ok();
                AppWrite::ok("验证成功".to_string())
            } else {
                AppWrite::error("验证失败".to_string())
            }
        },
        None => AppWrite::error("验证失败".to_string())
    }
}