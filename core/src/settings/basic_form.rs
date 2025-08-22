use crate::AppCore;
use anyhow::anyhow;
use database::entity::users;
use error::AppError;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use session::Session;

#[derive(Serialize, Clone, Debug)]
pub struct UserBasicSetting {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub company: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
    pub website_url: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SettingBasicFormParam {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub company: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
}

impl AppCore {
    pub async fn setting_basic_form_update(
        &self,
        param: SettingBasicFormParam,
        session: Session,
    ) -> Result<(), AppError> {
        let user = self.user_context(session).await?;
        let model = users::Entity::find_by_id(user.user_uid)
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Not found user")))?;
        let mut active = model.into_active_model();
        if let Some(display_name) = param.display_name {
            active.display_name = Set(Some(display_name));
        }
        if let Some(bio) = param.bio {
            active.bio = Set(Some(bio));
        }
        if let Some(location) = param.location {
            active.location = Set(Some(location));
        }
        if let Some(company) = param.company {
            active.company = Set(Some(company));
        }
        if let Some(timezone) = param.timezone {
            active.timezone = Set(Some(timezone));
        }
        if let Some(language) = param.language {
            active.language = Set(Some(language));
        }
        if let Some(theme) = param.theme {
            active.theme = Set(Some(theme));
        }
        active.updated_at = Set(Utc::now().naive_utc());
        active.update(&self.db).await?;
        Ok(())
    }

    pub async fn get_user_basic_setting(
        &self,
        session: Session,
    ) -> Result<UserBasicSetting, AppError> {
        let user = self.user_context(session).await?;
        let model = users::Entity::find_by_id(user.user_uid)
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Not found user")))?;

        Ok(UserBasicSetting {
            display_name: model.display_name,
            bio: model.bio,
            location: model.location,
            company: model.company,
            timezone: model.timezone,
            language: model.language,
            theme: model.theme,
            website_url: model.website_url,
            avatar: model.avatar_url,
        })
    }
}
