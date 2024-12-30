use crate::config::CFG;
use crate::emails::{Email, EmailMSG};
use lettre::message::Mailbox;

pub const EMAIL_TEMPLATE_EMAIL: &str = include_str!("../../template/captcha.html");
pub const EMAIL_TEMPLATE_USERS_FORGET_PASSWD: &str =
    include_str!("../../template/users_forgetpasswd.html");

impl Email {
    pub async fn send_captcha(&self, email: Mailbox, captcha: &str) {
        let tmp = EMAIL_TEMPLATE_EMAIL.to_string();
        let cfg = CFG.get().unwrap().email.clone();
        let tmp = tmp.replace("123456", captcha);
        self.send(EmailMSG {
            from: cfg.from.parse().unwrap(),
            reply: cfg.from.parse().unwrap(),
            to: email,
            subject: "GitData Captcha".to_string(),
            body: tmp,
        })
    }
    pub async fn send_forget_token(&self, email: Mailbox, token: String) {
        let tmp = EMAIL_TEMPLATE_USERS_FORGET_PASSWD.to_string();
        let cfg = CFG.get().unwrap().email.clone();
        let tmp = tmp.replace(
            "https://gitdata.ai/auth/UpPwd",
            &format!("https://gitdata.ai/auth/reset/{}", token),
        );
        self.send(EmailMSG {
            from: cfg.from.parse().unwrap(),
            reply: cfg.from.parse().unwrap(),
            to: email,
            subject: "GitData Reset You Password".to_string(),
            body: tmp,
        })
    }
}
