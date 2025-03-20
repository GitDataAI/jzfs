use crate::jobs::EmailJobs;
use crate::message::EmailMessage;
use async_iterator::Iterator;
use jz_jobs::{Queue, SeaOrmQueue};
use lazy_static::lazy_static;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use log::{error, info};
use std::str::FromStr;

lazy_static! {
    pub static ref SMTP_ADDR: String = "smtp.exmail.qq.com".to_string();
    pub static ref SMTP_PORT: u16 = 465;
    pub static ref SMTP_USERNAME: String =
        std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    pub static ref SMTP_PASSWORD: String =
        std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
}
mod template {
    pub const CAPTCHA: &str = include_str!("template/captcha.html");
    pub const USER_FOR_GET_PASSWD: &str = include_str!("template/users_forgetpasswd.html");
}

#[derive(Clone)]
pub struct EmailExecute {
    pub jobs: EmailJobs,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailExecute {
    pub async fn init(sea_orm_queue: SeaOrmQueue) -> Self {
        let creds = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_PASSWORD.to_owned());
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&SMTP_ADDR)
                .unwrap()
                .credentials(creds)
                .build();
        Self {
            jobs: EmailJobs::init(sea_orm_queue),
            mailer,
        }
    }
    pub fn run(&self) {
        let mut us = self.clone();
        tokio::spawn(async move {
            info!("start email jobs");
            loop {
                if let Ok(item) = us.jobs.jobs.pulls::<EmailMessage>("email",200).await {
                    for (id, msg) in item {
                        match msg {
                            EmailMessage::Captcha { email, code } => match us.captcha(email, code).await {
                                Ok(_) => {
                                    us.jobs.jobs.ok("email", id).await.ok();
                                }
                                Err(_) => {
                                    us.jobs.jobs.fail("email", id).await.ok();
                                }
                            },
                        };
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });
    }
    pub async fn captcha(&self, email: String, code: String) -> anyhow::Result<()> {
        let mail: Mailbox = Mailbox::from_str(&email)?;
        let email = Message::builder()
            .from(SMTP_USERNAME.to_owned().parse().unwrap())
            .reply_to(SMTP_USERNAME.to_owned().parse().unwrap())
            .to(mail)
            .subject("GitDataAI")
            .header(ContentType::TEXT_HTML)
            .body(template::CAPTCHA.replace("123456", &code))?;
        match self.mailer.send(email).await {
            Ok(_) => {
                info!("send email success");
                Ok(())
            }
            Err(e) => {
                error!("send email error: {}", e);
                Err(anyhow::anyhow!("send email error: {}", e))
            }
        }
    }
}
