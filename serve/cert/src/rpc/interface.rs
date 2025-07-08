use crate::rpc::session::UsersSession;
use crate::schema::{
    AccessKeySearch, AppResult, CertAuthLoginParam, CertEmailCaptchaParam, CertEmailCaptchaVerify,
    CertRegisterParam, SecurityEventRegisterParam, SshKeySearch,
};
use authd::{security, users};
use uuid::Uuid;

#[tarpc::service]
pub trait CertInterFace {
    async fn user_auth_login(param: CertAuthLoginParam) -> AppResult<UsersSession>;
    async fn user_auth_register(param: CertRegisterParam) -> AppResult<UsersSession>;
    ///No matter which component the verification code event occurs in\
    /// it is used as the occurrence and verification node
    async fn email_captcha(key: String, param: CertEmailCaptchaParam) -> AppResult<()>;
    /// Verify that the verification code is correct
    async fn email_verify(key: String, param: CertEmailCaptchaVerify) -> AppResult<bool>;

    /// When a security event occurs in a user account\
    /// an event message must be registered in the database\
    /// This type of event can occur in any component
    async fn security_event_register(param: SecurityEventRegisterParam) -> AppResult<Uuid>; // TODO

    /// Get a list of security events
    async fn security_event_list(users_uid: Uuid) -> AppResult<Vec<security::Model>>; // TODO

    /// Search for SSH keys
    /// When Git protoc of ssh is used, the public key is searched for
    async fn sshkey_search(param: SshKeySearch) -> AppResult<users::Model>; //  TODO
    /// Search for access keys
    /// When use Openapi or other app
    async fn access_key_search(param: AccessKeySearch) -> AppResult<users::Model>; //  TODO

    // TODO Consider adding mobile phone number verification

    async fn health_check() -> chrono::NaiveDateTime;
}
