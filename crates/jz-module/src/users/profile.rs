use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::Deserialize;
use uuid::Uuid;
use crate::AppModule;

#[derive(Deserialize)]
pub struct UpdateProfile {
    pub description: Option<String>,
    pub website: Vec<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub location: Option<String>,
}

impl AppModule {
    pub async fn profile_info_by_id(&self, uid: Uuid) -> anyhow::Result<serde_json::Value> {
        let user = self.user_info_by_id(uid).await?;
        Ok(serde_json::json!({
            "avatar": user.avatar,
            "uid": user.uid,
            "description": user.description,
            "website": user.website,
            "timezone": user.timezone,
            "language": user.language,
            "location": user.location,
            "theme": user.theme,
        }))
    }
    
    pub async fn profile_update(&self, ops_uid: Uuid, update: UpdateProfile) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(ops_uid).await?.into_active_model();
        if let Some(description) = update.description {
            user.description = sea_orm::ActiveValue::Set(Option::from(description));
        }
        if let Some(timezone) = update.timezone {
            user.timezone = sea_orm::ActiveValue::Set(Option::from(timezone));
        }
        if let Some(language) = update.language {
            user.language = sea_orm::ActiveValue::Set(Option::from(language));
        }
        if let Some(location) = update.location {
            user.location = sea_orm::ActiveValue::Set(Option::from(location));
        }
        if !update.website.is_empty() {
            user.website = sea_orm::ActiveValue::Set(update.website);
        }
        user.update(&self.write).await?;
        Ok(())
    }
    
    pub async fn profile_update_theme(&self, ops_uid: Uuid, theme: String) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(ops_uid).await?.into_active_model();
        user.theme = sea_orm::ActiveValue::Set(theme);
        user.update(&self.write).await?;
        Ok(())
    }
    
    pub async fn profile_update_password(&self, ops_uid: Uuid, password: String) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(ops_uid).await?.into_active_model();
        user.password = sea_orm::ActiveValue::Set(password);
        user.update(&self.write).await?;
        Ok(())
    }
    
    pub async fn profile_update_email(&self, ops_uid: Uuid, email: String) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(ops_uid).await?.into_active_model();
        user.email = sea_orm::ActiveValue::Set(email);
        user.update(&self.write).await?;
        Ok(())
    }
    pub async fn profile_update_avatar(&self, ops_uid: Uuid, avatar: String) -> anyhow::Result<()> {
        let mut user = self.user_info_by_id(ops_uid).await?.into_active_model();
        user.avatar = sea_orm::ActiveValue::Set(Option::from(avatar));
        user.update(&self.write).await?;
        Ok(())
    }
    
}