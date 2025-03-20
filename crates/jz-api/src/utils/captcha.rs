use actix_session::Session;
use actix_web::HttpResponse;
use captcha_rs::CaptchaBuilder;
use serde_json::json;

pub fn captcha_new(session: Session) -> anyhow::Result<String> {
    let captcha = CaptchaBuilder::new()
        .length(4)
        .width(160)
        .height(35)
        .dark_mode(false)
        .complexity(1)
        .compression(40)
        .build();
    let base64 = captcha.to_base64();
    let text = captcha.text;
    session.insert("captcha", text)?;
    Ok(base64)
}

pub fn captcha_check(session: Session, captcha: String) -> anyhow::Result<bool> {
    let text = session.get::<String>("captcha")?;
    if text.is_none() {
        return Ok(false);
    }
    let text = text.unwrap();
    Ok(text == captcha)
}

pub async fn captcha_check_actix(session: Session, captcha: String) -> Result<(), HttpResponse> {
    match captcha_check(session.clone(), captcha) {
        Ok(false) => Err(HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "验证码错误"
        }))),
        Ok(true) => Ok(()),
        Err(e) => Err(HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": e.to_string()
        }))),
    }
}
