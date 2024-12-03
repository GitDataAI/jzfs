pub mod captcha;
pub mod forgetpwd;
use crate::server::email::EmailServer;

#[derive(Clone)]
pub struct EmailService{
    pub(crate) server: EmailServer
}