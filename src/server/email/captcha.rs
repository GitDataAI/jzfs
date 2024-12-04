use lettre::message::Mailbox;
use crate::config::email::captcha::CAPTCHA;
use crate::config::file::CFG;
use crate::server::email::EmailServer;
use crate::server::email::msg::EmailMSG;

impl EmailServer {
    pub async fn send_captcha(&self, email: Mailbox, code: String){
        let tmp = CAPTCHA.to_string();
        let cfg = CFG.get().unwrap().email.clone();
        let tmp = tmp.replace("123456", &code);
        self.send(EmailMSG{
            from: cfg.from.parse().unwrap(),
            reply: cfg.from.parse().unwrap(),
            to: email,
            subject: "GitData Captcha".to_string(),
            body: tmp,
        })
    }
}