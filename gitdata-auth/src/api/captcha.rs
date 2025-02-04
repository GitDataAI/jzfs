use actix_web::{HttpRequest, Responder};
use captcha_rs::CaptchaBuilder;
use serde_json::json;
use sha256::Sha256Digest;
use actix_session::Session;
use lib_entity::write::AppWrite;

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
