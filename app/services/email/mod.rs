use std::io;
use std::sync::Arc;
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


#[derive(Clone)]
pub struct EmailEvent {
    tx : Arc<AsyncSmtpTransport<Tokio1Executor>>,
}

impl EmailEvent {
    pub async fn new() -> io::Result<EmailEvent> {
        let creds = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_PASSWORD.to_owned());
        let mailer : AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&SMTP_ADDR)
                .unwrap()
                .credentials(creds)
                .build();

        Ok(EmailEvent { tx : Arc::new(mailer) })
    }
    pub async fn send(&self, email_type : EmailType) {
        info!("Will Send Email to {:?}",email_type);
        match email_type {
            EmailType::Captcha(x) => {
                let email = Message::builder()
                    .from(SMTP_USERNAME.to_owned().parse().unwrap())
                    .reply_to(SMTP_USERNAME.to_owned().parse().unwrap())
                    .to(x.email.parse().unwrap())
                    .subject("GitData Code")
                    .header(ContentType::TEXT_HTML)
                    .body(CAPTCHA.replace("123456", &x.code))
                    .unwrap();
                match self.tx.send(email).await {
                    Ok(_) => info!("Email sent {} successfully!", x.email),
                    Err(e) => error!("Could not send email: {e:?}"),
                }
            }
        }
    }
}
