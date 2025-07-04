use sea_orm::DatabaseConnection;
use tarpc::context::Context;
use uuid::Uuid;
use session::storage::RedisStorage;
use crate::models::security::Model;
use crate::rpc::interface::CertInterFace;
use crate::rpc::session::UsersSession;
use crate::schema::{AccessKeySearch, AppResult, CertAuthLoginParam, CertEmailCaptchaParam, CertEmailCaptchaVerify, CertRegisterParam, SecurityEventRegisterParam, SshKeySearch};

#[derive(Clone)]
pub struct AppCertService {
    pub db: DatabaseConnection,
    pub cache: RedisStorage,
    pub mq: async_nats::client::Client,
}

impl AppCertService {
    pub const MQ_EMAIL_CAPTCHA_CHANNEL: &'static str = "captcha.email";
}
pub mod login;
pub mod register;
pub mod email_captcha;
pub mod email_verify;


impl CertInterFace for AppCertService {
    async fn user_auth_login(self, context: Context, param: CertAuthLoginParam) -> AppResult<UsersSession> {
        todo!()
    }

    async fn user_auth_register(self, context: Context, param: CertRegisterParam) -> AppResult<UsersSession> {
        todo!()
    }

    async fn email_captcha(self, context: Context, key: String, param: CertEmailCaptchaParam) -> AppResult<()> {
        todo!()
    }

    async fn email_verify(self, context: Context, key: String, param: CertEmailCaptchaVerify) -> AppResult<bool> {
        todo!()
    }

    async fn security_event_register(self, context: Context, param: SecurityEventRegisterParam) -> AppResult<Uuid> {
        todo!()
    }

    async fn security_event_list(self, context: Context, users_uid: Uuid) -> AppResult<Vec<Model>> {
        todo!()
    }

    async fn sshkey_search(self, context: Context, param: SshKeySearch) -> AppResult<crate::models::users::Model> {
        todo!()
    }

    async fn access_key_search(self, context: Context, param: AccessKeySearch) -> AppResult<crate::models::users::Model> {
        todo!()
    }
}