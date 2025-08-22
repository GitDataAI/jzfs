use crate::AppCore;
use database::entity::users;
use error::AppError;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use session::{Session, USER_KET, UserSession};
use sha256::Sha256Digest;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthUserLoginParam {
    pub username: String,
    pub password: String,
}

impl AppCore {
    pub async fn auth_user_login(
        &self,
        param: AuthUserLoginParam,
        session: Session,
    ) -> Result<UserSession, AppError> {
        let model = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(param.username.to_lowercase()))
                    .add(users::Column::Email.eq(param.username.to_lowercase())),
            )
            .filter(
                Condition::all()
                    .add(users::Column::PasswordHash.eq(param.password.to_lowercase().digest())),
            )
            .one(&self.db)
            .await
            .map_err(|e| AppError::from(e))?
            .ok_or(AppError::from(anyhow::anyhow!(
                "username or password is incorrect"
            )))?;
        let session_model = UserSession::from(model.clone());
        session.insert(USER_KET, session_model.clone()).ok();
        self.auth_user_login_after(model).await.ok();
        Ok(session_model)
    }
    async fn auth_user_login_after(&self, param: users::Model) -> Result<(), AppError> {
        let login_count = param.login_count.clone();
        let mut active = param.into_active_model();
        active.login_count = Set(login_count + 1);
        active.last_login_at = Set(Some(Utc::now().naive_utc()));
        active.update(&self.db).await?;
        Ok(())
    }
}
