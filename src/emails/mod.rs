use crate::config::CFG;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{error, info};
pub mod template;

#[derive(Clone)]
pub struct Email {
    rx: UnboundedSender<EmailMSG>,
}

#[derive(Clone)]
pub struct EmailMSG {
    pub from: Mailbox,
    pub reply: Mailbox,
    pub to: Mailbox,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub async fn init() -> Email {
        info!("Email Service starting.....");
        let (rx, mut tx) = tokio::sync::mpsc::unbounded_channel::<EmailMSG>();
        let cfg = CFG.get().unwrap().clone();

        tokio::spawn(async move {
            let creds =
                Credentials::new(cfg.email.username.to_owned(), cfg.email.password.to_owned());
            let mailer: AsyncSmtpTransport<Tokio1Executor> =
                AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.email.smtp)
                    .unwrap()
                    .credentials(creds)
                    .build();
            while let Some(tx) = tx.recv().await {
                let email = Message::builder()
                    .from(tx.from)
                    .reply_to(tx.reply)
                    .to(tx.to.clone())
                    .subject(tx.subject)
                    .header(ContentType::TEXT_HTML)
                    .body(tx.body)
                    .unwrap();
                match mailer.send(email).await {
                    Ok(_) => info!("Email sent {} successfully!", tx.to.to_string()),
                    Err(e) => error!("Could not send email: {e:?}"),
                }
            }
        });
        info!("Email Service started");
        Self { rx }
    }
    pub fn send(&self, msg: EmailMSG) {
        self.rx.send(msg).unwrap();
    }
}
