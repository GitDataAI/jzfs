use anyhow::anyhow;
use config::email::AppEmailConfig;
use error::AppError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::client::{Tls, TlsParameters},
};
use log::{error, info};
use tokio::sync::OnceCell;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

pub static EMAIL: OnceCell<EmailThread> = OnceCell::const_new();

pub struct EmailThread {
    pub config: AppEmailConfig,
    pub sender: UnboundedSender<EmailTask>,
    pub handle: JoinHandle<()>,
}

#[derive(Clone, Debug)]
pub struct EmailTask {
    pub target: Mailbox,
    pub content: String,
    pub subject: String,
}

impl EmailThread {
    pub async fn init(config: AppEmailConfig) {
        EMAIL
            .get_or_init(move || async { EmailThread::new(config).await })
            .await;
    }

    pub async fn new(config: AppEmailConfig) -> EmailThread {
        let cfg = config.clone();
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<EmailTask>();

        let smtp_host = &config.smtp_host;
        let smtp_port = config.smtp_port;
        let creds = Credentials::new(
            config.smtp_username.to_owned(),
            config.smtp_password.to_owned(),
        );

        let tls_params = TlsParameters::builder(smtp_host.to_string())
            .dangerous_accept_invalid_certs(false)
            .dangerous_accept_invalid_hostnames(false)
            .build()
            .map_err(|e| {
                error!("TLS configuration error: {}", e);
                e
            })
            .unwrap();

        let mailer = if smtp_port == 465 {
            AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
                .expect("Failed to create SMTP connection")
                .port(smtp_port)
                .credentials(creds)
                .tls(Tls::Wrapper(tls_params))
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
                .expect("Failed to create SMTP connection")
                .port(smtp_port)
                .credentials(creds)
                .tls(Tls::Opportunistic(tls_params))
                .build()
        };

        let handle = tokio::spawn(async move {
            info!("Email thread started");
            while let Some(task) = receiver.recv().await {
                let from_addr: Mailbox = match config.smtp_username.parse() {
                    Ok(addr) => addr,
                    Err(e) => {
                        error!("Failed to parse sender address: {}", e);
                        continue;
                    }
                };

                let email = match Message::builder()
                    .from(from_addr.clone())
                    .reply_to(from_addr)
                    .to(task.target.clone())
                    .subject(task.subject)
                    .header(ContentType::TEXT_HTML)
                    .body(task.content)
                {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Failed to construct email: {}", e);
                        continue;
                    }
                };

                match mailer.send(email).await {
                    Ok(_) => {
                        info!("Email sent successfully to {:?}", task.target);
                    }
                    Err(e) => {
                        error!("Failed to send email: {}", e);
                    }
                }
            }
        });

        Self {
            config: cfg.clone(),
            sender,
            handle,
        }
    }

    pub async fn sender(task: EmailTask) -> Result<(), AppError> {
        let email_thread = EMAIL
            .get_or_try_init(|| async {
                error!("Email thread not initialized");
                Err(AppError::from(anyhow!("Email thread not initialized")))
            })
            .await?;

        email_thread
            .sender
            .send(task)
            .map_err(|_| AppError::from(anyhow!("Failed to send email task to channel")))?;

        Ok(())
    }
}
