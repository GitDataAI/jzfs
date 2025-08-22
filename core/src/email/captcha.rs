use crate::AppCore;
use crate::email::email_thread::{EmailTask, EmailThread};
use crate::email::{ALLOW_NEXT, CAPTCHA_KET, CAPTCHA_TEMPLATE};
use anyhow::anyhow;
use error::AppError;
use lettre::Address;
use lettre::message::Mailbox;
use rand::random;
use serde::{Deserialize, Serialize};
use session::Session;
use std::str::FromStr;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailCaptchaSendParam {
    email: String,
    code: String,
}

impl AppCore {
    pub async fn email_captcha_send(
        &self,
        param: EmailCaptchaSendParam,
        session: Session,
    ) -> Result<(), AppError> {
        let Ok(email) = Address::from_str(&param.email) else {
            return Err(AppError::from(anyhow!("Invalid email address")));
        };
        let email = Mailbox::new(None, email);
        let captcha: u32 = random::<u32>() & 999999;
        let captcha = format!("{:06}", captcha);

        let captcha_text = CAPTCHA_TEMPLATE.replace("{{code}}", &captcha);
        let task = EmailTask {
            target: email,
            content: captcha_text,
            subject: "GitDataAI | Captcha".to_string(),
        };
        session
            .insert(
                CAPTCHA_KET.to_string(),
                format!("{}-{}", param.email, captcha),
            )
            .map_err(|e| AppError::from(anyhow!(e)))?;
        EmailThread::sender(task).await?;
        Ok(())
    }
    pub async fn email_captcha_verify(
        &self,
        param: EmailCaptchaSendParam,
        session: Session,
    ) -> Result<(), AppError> {
        let captcha = session
            .get::<String>(&*CAPTCHA_KET.to_string())
            .map_err(|e| AppError::from(anyhow!(e)))?;
        if captcha.is_none() {
            return Err(AppError::from(anyhow!("Captcha not found")));
        }
        if captcha.unwrap() != format!("{}-{}", param.email, param.code) {
            return Err(AppError::from(anyhow!("Captcha error")));
        }
        session.remove(&*CAPTCHA_KET.to_string());
        session
            .insert(ALLOW_NEXT, true)
            .map_err(|e| AppError::from(anyhow!(e)))?;
        Ok(())
    }
}
