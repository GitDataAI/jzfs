use session::redis;
use crate::schema::{AppResult, CertEmailCaptchaVerify, MqEmailCode};
use crate::service::AppCertService;

impl AppCertService {
    pub async fn service_email_verify(&self, key: String, param: CertEmailCaptchaVerify) -> AppResult<bool> {
        let Ok(captcha) = self.cache.execute_command::<String>(redis::cmd("GET").arg(key).clone()).await else {
            return AppResult {
                code: 500,
                data: None,
                msg: Some("Redis error".to_string()),
            };
        };
        if let Ok(desert) = serde_json::from_str::<MqEmailCode>(&captcha) {
            if desert.email == param.email && desert.captcha == param.captcha {
                return AppResult {
                    code: 200,
                    data: Some(true),
                    msg: None,
                };
            }
        }
        AppResult {
            code: 200,
            data: Some(false),
            msg: None,
        }
    }
}