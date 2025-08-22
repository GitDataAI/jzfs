use crate::AppCore;
use crate::email::ALLOW_NEXT;
use database::entity::users;
use error::AppError;
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use session::{Session, USER_KET, UserSession};
use sha256::Sha256Digest;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthUserRegisterParam {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl AppCore {
    pub async fn auth_user_register_before(
        &self,
        param: AuthUserRegisterParam,
    ) -> Result<(), AppError> {
        match users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(param.username.clone()))
                    .add(users::Column::Email.eq(param.email.clone())),
            )
            .all(&self.db)
            .await
            .map_err(|e| AppError::from(e))
        {
            Ok(users) => {
                if users.len() > 0 {
                    return Err(AppError::from(anyhow::anyhow!(
                        "the username or email address already exists"
                    )));
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub async fn auth_user_register(
        &self,
        param: AuthUserRegisterParam,
        session: Session,
    ) -> Result<(), AppError> {
        if let Ok(Some(allow_next)) = session.get::<bool>(ALLOW_NEXT) {
            if !allow_next {
                return Err(AppError::from(anyhow::anyhow!(
                    "Please verify the email address"
                )));
            }
        } else {
            return Err(AppError::from(anyhow::anyhow!(
                "Please verify the email address"
            )));
        }
        session.remove(ALLOW_NEXT);
        self.auth_user_register_before(param.clone()).await?;
        let active = users::ActiveModel {
            uid: Set(Uuid::now_v7()),
            username: Set(param.username.to_lowercase().clone()),
            email: Set(param.email.to_lowercase()),
            password_hash: Set(param.password.to_lowercase().digest()),
            display_name: Set(Some(param.username)),
            avatar_url: Set(None),
            bio: Set(None),
            location: Set(None),
            website_url: Set(None),
            company: Set(None),
            is_active: Set(true),
            is_verified: Set(false),
            is_premium: Set(false),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            last_login_at: Set(Some(Utc::now().naive_utc())),
            timezone: Set(None),
            language: Set(None),
            theme: Set(None),
            login_count: Set(0),
        };
        let model = active.insert(&self.db).await?;
        let session_model = UserSession::from(model);
        session.insert(USER_KET, session_model).ok();
        Ok(())
    }
}
