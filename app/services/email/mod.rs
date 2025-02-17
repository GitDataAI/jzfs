use std::io;
use std::sync::Arc;
use async_static::async_static;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use lazy_static::lazy_static;
use crate::app::services::email::capctah::EmailCaptcha;

pub const CAPTCHA : &str = include_str!("captcha.html");
pub const USER_FOR_GET_PASSWD : &str = include_str!("users_forgetpasswd.html");


pub mod capctah;

#[derive(Debug, Serialize, Deserialize)]
pub enum EmailType {
    Captcha(EmailCaptcha),
}

lazy_static! {
    pub static ref SMTP_ADDR: String = "smtp.exmail.qq.com".to_string();
    pub static ref SMTP_PORT: u16 = 465;
    pub static ref SMTP_USERNAME: String = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    pub static ref SMTP_PASSWORD: String = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
}

async_static! {
    pub static ref SMTP: EmailEvent = EmailEvent::new().await.unwrap();
}

#[derive(Clone)]
pub struct EmailEvent {
    tx : Arc<tokio::sync::mpsc::UnboundedSender<EmailType>>,
}

impl EmailEvent {
    pub async fn new() -> io::Result<EmailEvent> {
        let creds = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_PASSWORD.to_owned());
        let mailer : AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&SMTP_ADDR)
                .unwrap()
                .credentials(creds)
                .build();

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            EmailEvent::listen(rx, mailer).await;
        });
        Ok(EmailEvent { tx : Arc::new(tx) })
    }
    pub async fn send(&self, email_type : EmailType) {
        self.tx.send(email_type).ok();
    }
    pub async fn listen(
        mut rx : tokio::sync::mpsc::UnboundedReceiver<EmailType>,
        mailer : AsyncSmtpTransport<Tokio1Executor>,
    ) {
        while let Some(email_type) = rx.recv().await {
            match email_type {
                EmailType::Captcha(x) => {
                    let email = Message::builder()
                        .from(SMTP_USERNAME.to_owned().parse().unwrap())
                        .reply_to(SMTP_USERNAME.to_owned().parse().unwrap())
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
