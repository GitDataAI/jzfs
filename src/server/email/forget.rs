use crate::server::email::msg::EmailMSG;
use crate::server::email::EmailServer;
use lettre::message::Mailbox;
use crate::config::CFG;
use crate::template::email::FORGET_EMAIL;

impl EmailServer {
    pub async fn send_forget_token(&self, email: Mailbox, token: String){
        let tmp = FORGET_EMAIL.to_string();
        let cfg = CFG.get().unwrap().email.clone();
        let tmp = tmp.replace("https://gitdata.ai/auth/UpPwd", &format!("https://gitdata.ai/auth/reset/{}",token));
        self.send(EmailMSG{
            from: cfg.from.parse().unwrap(),
            reply: cfg.from.parse().unwrap(),
            to: email,
            subject: "GitData Reset You Password".to_string(),
            body: tmp,
        })
    }
}