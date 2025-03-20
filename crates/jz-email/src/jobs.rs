use crate::message::EmailMessage;
use jz_jobs::{Queue, SeaOrmQueue};

#[derive(Clone)]
pub struct EmailJobs {
    pub jobs: SeaOrmQueue,
}

impl EmailJobs {
    pub fn init(jobs: SeaOrmQueue) -> EmailJobs {
        let us = EmailJobs { jobs };
        us
    }
    pub async fn send_email(&self, message: EmailMessage) -> anyhow::Result<()> {
        self.jobs.push("email", message).await?;
        Ok(())
    }
    
}

impl async_iterator::Iterator for EmailJobs {
    type Item = (String, EmailMessage);

    async fn next(&mut self) -> Option<Self::Item> {
        let result = self.jobs.pull::<EmailMessage>("email").await;
        match result {
            Ok(Some(message)) => Some(message),
            Ok(None) => None,
            Err(err) => {
                eprintln!("Error pulling message from queue: {}", err);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_iterator::Iterator;
    use sea_orm::Database;
    #[tokio::test]
    async fn test_email_jobs() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to database");
        let jobs = SeaOrmQueue::new(db, "email".to_string());
        jobs.init().await.expect("Failed to init jobs");
        let email_jobs = EmailJobs::init(jobs);
        let message = EmailMessage::Captcha {
            email: "test@example.com".to_string(),
            code: "1234".to_string(),
        };
        email_jobs.send_email(message.clone()).await.unwrap();
        let mut iter = email_jobs;
        while let Some(x) = iter.next().await {
            assert_eq!(x.1, message);
            break;
        }
    }
}
