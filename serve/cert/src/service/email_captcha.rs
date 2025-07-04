use rand::Rng;
use bytes::Bytes;
use session::redis;
use crate::schema::{AppResult, CertEmailCaptchaParam, MqEmailCode};
use crate::service::AppCertService;

impl AppCertService {
    pub async fn service_email_captcha(&self, key: String, param: CertEmailCaptchaParam) -> AppResult<()> {
        let captcha_length = param.length.unwrap_or(6);
        let random_captcha = rand::rng()
            .sample_iter(&rand::distr::Alphanumeric)
            .take(captcha_length)
            .map(char::from)
            .collect::<String>();
        
        let Ok(serde) = serde_json::to_string(&MqEmailCode {
            email: param.email,
            captcha: random_captcha.clone(),
        }) else { 
            return AppResult {
                code: 500,
                data: None,
                msg: Some("Serde Error".to_string()),
            };
        };
        let payload = Bytes::from(serde.clone());
        if let Err(e) = self.mq.publish(Self::MQ_EMAIL_CAPTCHA_CHANNEL,payload).await {
            return AppResult {
                code: 500,
                data: None,
                msg: Some(format!("Mq Error:{}",e).to_string()),
            };
        }
        if let Err(e) = self.cache.execute_command::<()>(redis::cmd("SET")
            .arg(key)
            .arg(serde)
            .arg("EX")
            .arg(300).clone()).await {
            return AppResult {
                code: 500,
                data: None,
                msg: Some(format!("Redis error:{}",e).to_string()),
            };
        }
        AppResult {
            code: 200,
            data: None,
            msg: None,
        }
    }
}