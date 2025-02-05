use std::io;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use log::{error, info};
use lib_config::AppNacos;
use lib_config::config::email::EmailConfig;
use lib_config::public::CAPTCHA;
use crate::server::email::captcha::EmailCaptcha;

pub mod captcha;

#[derive(Debug,Serialize,Deserialize)]
pub enum EmailType {
    Captcha(EmailCaptcha),
}


#[derive(Clone)]
pub struct EmailEvent {
    tx: Arc<tokio::sync::mpsc::UnboundedSender<EmailType>>,
}

impl EmailEvent {
    pub async fn new(nacos: AppNacos) -> io::Result<EmailEvent> {
        let config = nacos.config.get_email_config("main").await?;

        let creds = Credentials::new(config.username.to_owned(), config.password.to_owned());
        let mailer : AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp)
                .unwrap()
                .credentials(creds)
                .build();

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            EmailEvent::listen(config,rx, mailer).await;
        });
        Ok(EmailEvent {
            tx: Arc::new(tx),
        })
    }
    pub async fn send(&self, email_type: EmailType) {
        self.tx.send(email_type).ok();
    }
    pub async fn listen(config: EmailConfig, mut rx: tokio::sync::mpsc::UnboundedReceiver<EmailType>,mailer: AsyncSmtpTransport<Tokio1Executor>) {
        while let Some(email_type) = rx.recv().await {
            match email_type {
                EmailType::Captcha(x) => {
                    let email = Message::builder()
                        .from(config.from.parse().unwrap())
                        .reply_to(config.from.parse().unwrap())
                        .to(x.email.parse().unwrap())
                        .subject("GitData Code")
                        .header(ContentType::TEXT_HTML)
                        .body(CAPTCHA.replace("123456", &*x.code))
                        .unwrap();
                    match mailer.send(email).await {
                        Ok(_) => info!("Email sent {} successfully!", x.email),
                        Err(e) => error!("Could not send email: {e:?}"),
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_event() {
        let nacos = AppNacos::from_env().unwrap();
        let email_event = EmailEvent::new(nacos).await.unwrap();
        email_event.send(EmailType::Captcha(EmailCaptcha {
            email: "434836402@qq.com".to_string(),
            code: "123456".to_string(),
        })).await;
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
    #[test]
    fn test_captcha() {
        let captcha = EmailCaptcha::generate_captcha("256".to_string());
        assert_eq!(captcha.email, "256".to_string());
        let event = EmailType::Captcha(captcha);
        println!("{}", serde_json::to_string(&event).unwrap());
    }
}
