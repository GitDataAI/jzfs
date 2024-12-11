pub mod captcha;
pub mod bind;
pub mod forgetpwd;

use sea_orm::DatabaseConnection;
use crate::server::email::EmailServer;

#[derive(Clone)]
pub struct EmailService{
    pub db: DatabaseConnection,
    pub(crate) server: EmailServer
}