use crate::server::email::msg::EmailMSG;
use crate::server::email::EmailServer;
use lettre::message::Mailbox;
use crate::config::email::forget::FORGET_EMAIL;

impl EmailServer {
    pub async fn send_forget_token(&self, email: Mailbox, token: String){
        let tmp = FORGET_EMAIL.to_string();
        let tmp = tmp.replace("https://gitdata.ai/auth/UpPwd", &format!("https://gitdata.ai/auth/reset/{}",token));
        self.send(EmailMSG{
            from: "gitdata-bot@gitdata.ai".parse().unwrap(),
            reply: "gitdata-bot@gitdata.ai".parse().unwrap(),
            to: email,
            subject: "GitData Reset You Password".to_string(),
            body: tmp,
        })
    }
}