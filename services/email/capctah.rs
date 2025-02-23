use crate::services::email::EmailType;
use crate::services::AppState;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailCaptcha {
    pub email : String,
    pub code : String,
}

impl EmailCaptcha {
    pub fn generate_captcha(email : String) -> Self {
        let rand = rand::random::<u32>() % 100000;
        let code = rand.to_string();
        EmailCaptcha { email, code }
    }
}

impl AppState {
    pub async fn email_captcha(&self, email: String) -> io::Result<EmailCaptcha> {
        let captcha = EmailCaptcha::generate_captcha(email);
        self.email.send(EmailType::Captcha(captcha.clone())).await;
        Ok(captcha)
    }
}