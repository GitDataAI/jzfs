use actix_web::HttpResponse;
use serde_json::json;
use uuid::Uuid;
use cert::schema::{CertEmailCaptchaParam, CertEmailCaptchaVerify};
use web_session::builder::WebSession;
use crate::endpoint::Endpoint;

impl Endpoint {
    pub async fn email_captcha_send(&self, param: CertEmailCaptchaParam, web_session: WebSession) -> HttpResponse {
        let key = Uuid::now_v7().to_string();
        web_session.0.set(WebSession::USER_EMAIL_CAPTCHA, key.clone());
        let res = self.cert.email_captcha(self.new_context(), key, param).await;
        match res {
            Ok(res) => HttpResponse::Ok().json(json!({ "code": res.code })),
            Err(err) => HttpResponse::Ok().json(json!({ "code": 501, "msg": err.to_string() })),
        }
    }
    pub async fn email_captcha_verify(&self, param: CertEmailCaptchaVerify, web_session: WebSession) -> HttpResponse {
        let Ok(key) = web_session.0.get::<String>(WebSession::USER_EMAIL_CAPTCHA) else {
            return HttpResponse::Ok().json(json!({ "code": 501, "msg": "email captcha not found" }));
        };
        let res = self.cert.email_verify(self.new_context(), key, param).await;
        match res {
            Ok(res) => {
                if let Some(data) = res.data {
                    if data {
                        HttpResponse::Ok().json(json!({ "code": 200, "msg": "success" }))
                    } else {
                        HttpResponse::Ok().json(json!({ "code": 401, "msg": "email verify error" }))
                    }
                } else {
                    HttpResponse::Ok().json(json!({ "code": 500, "msg": res.msg }))
                }
            },
            Err(err) => HttpResponse::Ok().json(json!({ "code": 501, "msg": err.to_string() })),
        }
    }
}