use authd::security::Model;
use crate::rpc::interface::CertInterFace;
use crate::rpc::session::UsersSession;
use crate::schema::{
    AccessKeySearch, AppResult, CertAuthLoginParam, CertEmailCaptchaParam, CertEmailCaptchaVerify,
    CertRegisterParam, SecurityEventRegisterParam, SshKeySearch,
};
use sea_orm::DatabaseConnection;
use session::storage::RedisStorage;
use tarpc::context::Context;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppCertService {
    pub db: DatabaseConnection,
    pub cache: RedisStorage,
    pub mq: async_nats::client::Client,
}

impl AppCertService {
    pub const MQ_EMAIL_CAPTCHA_CHANNEL: &'static str = "captcha.email";
}
mod access_key_search;
pub mod email_captcha;
pub mod email_verify;
pub mod login;
pub mod register;
mod security_event_list;
mod security_event_register;
mod sshkey_search;

impl CertInterFace for AppCertService {
    async fn user_auth_login(
        self,
        _: Context,
        param: CertAuthLoginParam,
    ) -> AppResult<UsersSession> {
        self.auth_user_login(param).await
    }

    async fn user_auth_register(
        self,
        _: Context,
        param: CertRegisterParam,
    ) -> AppResult<UsersSession> {
        self.auth_user_register(param).await
    }

    async fn email_captcha(
        self,
        _context: Context,
        key: String,
        param: CertEmailCaptchaParam,
    ) -> AppResult<()> {
        self.service_email_captcha(key, param).await
    }

    async fn email_verify(
        self,
        _context: Context,
        key: String,
        param: CertEmailCaptchaVerify,
    ) -> AppResult<bool> {
        self.service_email_verify(key, param).await
    }

    async fn security_event_register(
        self,
        _context: Context,
        param: SecurityEventRegisterParam,
    ) -> AppResult<Uuid> {
        self.service_security_event_register(param).await
    }

    async fn security_event_list(self, context: Context, users_uid: Uuid) -> AppResult<Vec<Model>> {
        self.service_security_event_list(users_uid).await
    }

    async fn sshkey_search(
        self,
        _context: Context,
        param: SshKeySearch,
    ) -> AppResult<authd::users::Model> {
        self.service_sshkey_search(param).await
    }

    async fn access_key_search(
        self,
        _context: Context,
        param: AccessKeySearch,
    ) -> AppResult<authd::users::Model> {
        self.service_access_key_search(param).await
    }
}
