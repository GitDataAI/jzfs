use crate::AppCore;
use anyhow::anyhow;
use database::entity::users;
use error::AppError;
use sea_orm::EntityTrait;
use sea_orm::prelude::Uuid;
use session::{Session, USER_KET, UserSession};

const COUNT: &str = "click_count";

impl AppCore {
    pub async fn user_context_current(&self, session: Session) -> Result<UserSession, AppError> {
        if let Ok(Some(user_session)) = session.get::<UserSession>(USER_KET) {
            if let Ok(Some(count)) = session.get::<i64>(COUNT) {
                if count % 10 == 0 {
                    let users = self.user_context_find_by_uid(user_session.user_uid).await?;
                    session.insert(COUNT, 1).ok();
                    session
                        .insert(USER_KET, UserSession::from(users.clone()))
                        .ok();
                    Ok(UserSession::from(users))
                } else {
                    session.insert(COUNT, count + 1).ok();
                    Ok(user_session)
                }
            } else {
                session.insert(COUNT, 1).ok();
                Ok(user_session)
            }
        } else {
            Err(AppError::from(anyhow!("Not login")))
        }
    }

    pub async fn user_context_find_by_uid(&self, user_uid: Uuid) -> Result<users::Model, AppError> {
        Ok(users::Entity::find_by_id(user_uid)
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Not found")))?)
    }
    pub async fn user_context(&self, session: Session) -> Result<UserSession, AppError> {
        if let Ok(Some(user_session)) = session.get::<UserSession>(USER_KET) {
            Ok(user_session)
        } else {
            Err(AppError::from(anyhow!("Not login")))
        }
    }
}
