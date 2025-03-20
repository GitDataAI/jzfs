use crate::AppModule;
use jz_email::execute::EmailExecute;
use jz_email::message::EmailMessage;

impl AppModule {
    pub async fn send_email_captcha(&self, email: String) -> anyhow::Result<String> {
        let mut rand = (rand::random::<u32>() % 999999).to_string();
        if rand.len() < 6 {
            for _ in 0..(6 - rand.len()) {
                rand = "0".to_string() + &rand;
            }
        }
        let email_execute = self.ioc.take::<EmailExecute>().await
            .ok_or(anyhow::anyhow!("email execute not found"))?;
        email_execute.jobs.send_email(EmailMessage::Captcha {
            email: email.clone(),
            code: rand.to_string(),
        }).await?;
        Ok(rand.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_send_email_captcha(){
        let module = crate::AppModule::init_env()
            .await
            .expect("init module error");
        for _ in 0..1 {
            module.send_email_captcha("434836402@qq.com".to_string()).await.expect("send email error");
        }
        tokio::time::sleep(std::time::Duration::from_secs(20)).await;
        
    }
}