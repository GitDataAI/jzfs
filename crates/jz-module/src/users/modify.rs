use crate::AppModule;
use sea_orm::*;
use serde::Deserialize;
use sha256::Sha256Digest;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserModify {
    website: Option<Vec<String>>,
    location: Option<String>,
    timezone: Option<String>,
    theme: Option<String>,
    language: Option<String>,
    description: Option<String>,
}

impl AppModule {
    pub async fn user_modify_password(
        &self,
        users_uid: Uuid,
        password: String,
    ) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(users_uid).await?.into_active_model();
        user.password = Set(password.digest());
        user.updated_at = Set(chrono::Local::now().naive_local());
        user.update(&self.write).await?;
        Ok(())
    }
    pub async fn user_modify_email(&self, users_uid: Uuid, email: String) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(users_uid).await?.into_active_model();
        user.email = Set(email);
        user.updated_at = Set(chrono::Local::now().naive_local());
        user.update(&self.write).await?;
        Ok(())
    }
    pub async fn user_modify_avatar(&self, users_uid: Uuid, avatar: String) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(users_uid).await?.into_active_model();
        user.avatar = Set(Option::from(avatar));
        user.updated_at = Set(chrono::Local::now().naive_local());
        user.update(&self.write).await?;
        Ok(())
    }
    pub async fn users_modify_config(
        &self,
        users_uid: Uuid,
        param: UserModify,
    ) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(users_uid).await?.into_active_model();

        if let Some(website) = param.website {
            user.website = Set(website);
        }
        if let Some(location) = param.location {
            user.location = Set(Option::from(location));
        }
        if let Some(timezone) = param.timezone {
            user.timezone = Set(Option::from(timezone));
        }
        if let Some(theme) = param.theme {
            user.theme = Set(theme);
        }
        if let Some(language) = param.language {
            user.language = Set(Option::from(language));
        }
        if let Some(description) = param.description {
            user.description = Set(Option::from(description));
        }
        user.updated_at = Set(chrono::Local::now().naive_local());
        user.update(&self.write).await?;
        Ok(())
    }
}
