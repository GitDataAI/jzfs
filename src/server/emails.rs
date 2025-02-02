use crate::error::{JZError, JZResult};
use crate::server::MetaData;
use actix_session::Session;
use lettre::message::Mailbox;
use rand::Rng;
use std::str::FromStr;

impl MetaData {
    pub async fn email_captcha(&self, email: &str, session: Session) -> JZResult<()> {
        let email = email.to_string();
        let email = match Mailbox::from_str(&email) {
            Ok(email) => email,
            Err(err) => return Err(JZError::Other(anyhow::anyhow!(err))),
        };
        let mut rng = rand::rng();
        let captcha: String = (0..6)
            .map(|_| rng.random_range(0..10).to_string())
            .collect();
        session.insert("captcha", captcha.clone()).ok();
        self.email.send_captcha(email, &captcha).await;
        Ok(())
    }
    pub async fn email_captcha_check(&self, captcha: &str, session: Session) -> JZResult<()> {
        let captchas = captcha.to_string();
        let captcha = match session.get::<String>("captcha") {
            Ok(captcha) => captcha,
            Err(err) => return Err(JZError::Other(anyhow::anyhow!(err))),
        };
        if captcha.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("captcha not found")));
        }
        let captcha = captcha.unwrap();
        if captcha != captchas {
            return Err(JZError::Other(anyhow::anyhow!("captcha error")));
        }
        session.insert("Check", true).ok();
        Ok(())
    }
}
