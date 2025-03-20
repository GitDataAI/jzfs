use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn utils_captcha(session: Session) -> impl Responder {
    match crate::utils::captcha::captcha_new(session) {
        Ok(captcha) => HttpResponse::Ok().json(json!({
            "code": 0,
            "data": {
                "captcha": captcha,
            }
        })),
        Err(e) => HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": e.to_string()
        })),
    }
}
