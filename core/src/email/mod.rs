pub mod email_thread;

pub mod captcha;

pub const CAPTCHA_KET: &str = "captcha";
pub const CAPTCHA_TEMPLATE: &str = include_str!("./template/captcha.html");
pub const ALLOW_NEXT: &str = "allow_next";
