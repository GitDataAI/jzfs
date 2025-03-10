use std::io;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::Deserialize;
use uuid::Uuid;
use crate::services::AppState;

#[derive(Deserialize)]
pub struct UserConfigUploadParam {
    name: Option<String>,
    description: Option<String>,
    email: Option<String>,
    location: Option<String>,
    website: Option<String>,
    language: Option<String>,
    timezone: Option<String>,
    topic: Vec<String>
}


impl AppState {
    pub async fn users_upload_config(&self, users_uid: Uuid, param: UserConfigUploadParam) -> io::Result<()> {
        let users = self.user_info_by_uid(users_uid).await?;
        let mut users = users.into_active_model();
        if let Some(name) = param.name {
            users.name = sea_orm::ActiveValue::Set(name);
        }
        if let Some(description) = param.description {
            users.description = sea_orm::ActiveValue::Set(Option::from(description));
        }
        if let Some(email) = param.email {
            users.email = sea_orm::ActiveValue::Set(email);
        }
        if let Some(location) = param.location {
            users.location = sea_orm::ActiveValue::Set(Option::from(location));
        }
        if let Some(website) = param.website {
            users.website = sea_orm::ActiveValue::Set(Option::from(website));
        }
        if let Some(timezone) = param.timezone {
            users.timezone = sea_orm::ActiveValue::Set(Option::from(timezone));
        }
        if let Some(language) = param.language {
            users.language = sea_orm::ActiveValue::Set(Option::from(language));
        }
        users.topic = sea_orm::ActiveValue::Set(param.topic);
        users.updated_at = sea_orm::ActiveValue::Set(chrono::Local::now().naive_local());
        users.update(&self.write).await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }
}