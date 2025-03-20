#![feature(async_iterator)]
#![allow(unused)]

pub mod execute;
pub mod jobs;
pub mod message;

#[cfg(test)]
mod tests {
    use crate::execute::EmailExecute;
    use crate::message;
    use jz_jobs::{Queue, SeaOrmQueue};
    use sea_orm::Database;

    #[tokio::test]
    async fn test_email_jobs() -> anyhow::Result<()> {
        tracing_subscriber::fmt().init();
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to database");
        let jobs = SeaOrmQueue::new(db, "email".to_string());
        jobs.init().await?;
        let execute = EmailExecute::init(jobs).await;
        execute.run();
        execute
            .jobs
            .send_email(message::EmailMessage::Captcha {
                email: "434836402@qq.com".to_string(),
                code: "1233211234567".to_string(),
            })
            .await?;
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        Ok(())
    }
}
