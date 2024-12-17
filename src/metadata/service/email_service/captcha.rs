use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::metadata::service::email_service::EmailService;

impl EmailService {
    pub async fn generate_and_send_captcha(&self, email: String) -> anyhow::Result<String>{
        let rng = rand::thread_rng();
        let code: String = rng.sample_iter(&Alphanumeric).take(6).map(char::from).collect();
        self.email.send_captcha(email.parse().map_err(|e|{
            log::error!("[Error] {}", e);
            anyhow::anyhow!("[Email Err] {}", e)
        })?, code.clone()).await;
        Ok(code)
    }
}