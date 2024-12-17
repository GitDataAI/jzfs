use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::OnceCell;
use tracing::{error, info};
use crate::config::CFG;
use crate::server::email::msg::EmailMSG;

pub mod msg;
pub mod captcha;
pub mod forget;

pub static EMAIL_SERVICE: OnceCell<EmailServer> = OnceCell::const_new();


pub async fn init_email(){
    EMAIL_SERVICE.get_or_init(||async {
        EmailServer::init().await
    }).await;
}


#[derive(Clone)]
pub struct EmailServer{
    rx: UnboundedSender<EmailMSG>,
}


impl EmailServer {
    pub async fn init() -> EmailServer {
        info!("Email Service starting.....");
        let (rx, mut tx) = tokio::sync::mpsc::unbounded_channel::<EmailMSG>();
        let cfg = CFG.get().unwrap().clone();

        tokio::spawn(async move {
            let creds = Credentials::new(cfg.email.username.to_owned(), cfg.email.password.to_owned());
            let mailer: AsyncSmtpTransport<Tokio1Executor> =
                AsyncSmtpTransport::<Tokio1Executor>::relay(&*cfg.email.smtp)
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
        Self{
            rx
        }
    }
    pub fn send(&self, msg: EmailMSG){
        self.rx.send(msg).unwrap();
    }
}


#[cfg(test)]
mod email_test{
    use crate::server::email::EmailServer;

    #[tokio::test]
    async fn test_email_code(){
        let email = EmailServer::init().await;
        email.send_captcha(
            "3476561861@qq.com".to_string().parse().unwrap(),
            "123456".to_string()
        ).await;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    #[tokio::test]
    async fn test_email_forget(){
        let email = EmailServer::init().await;
        email.send_forget_token(
            "3476561861@qq.com".to_string().parse().unwrap(),
            "123456".to_string()
        ).await;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}