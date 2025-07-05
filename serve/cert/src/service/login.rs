use crate::models::users;
use crate::rpc::session::UsersSession;
use crate::schema::{AppResult, CertAuthLoginParam};
use crate::service::AppCertService;
use chrono::Utc;
use sea_orm::*;
use sha256::Sha256Digest;

impl AppCertService {
    pub async fn auth_user_login(&self, param: CertAuthLoginParam) -> AppResult<UsersSession> {
        let users = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Email.eq(&param.username))
                    .add(users::Column::Username.eq(&param.username)),
            )
            .filter(users::Column::Password.eq(&param.password.digest()))
            .one(&self.db)
            .await;
        match users {
            Ok(Some(user)) => {
                let session = UsersSession::from(user.clone());
                let mut user_active = user.into_active_model();
                user_active.last_login_at = Set(Some(Utc::now().naive_local()));
                user_active.update(&self.db).await.ok();
                AppResult {
                    code: 200,
                    data: Some(session),
                    msg: None,
                }
            }
            Ok(None) => AppResult {
                code: 400,
                data: None,
                msg: Some("Login Failed: Wrong username or password".to_string()),
            },
            Err(e) => AppResult {
                code: 500,
                data: None,
                msg: Some(e.to_string()),
            },
        }
    }
}
