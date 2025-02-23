use std::io;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::services::AppState;
use crate::model::users::users;

#[derive(Deserialize,Serialize)]
pub struct UserUpdateOptional {
    pub description: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
}


impl AppState {
    pub async fn user_update_optional(
        &self,
        uid: Uuid,
        parma: UserUpdateOptional,
    ) -> io::Result<users::Model> {
        let user = self.user_info_by_uid(uid).await?;
        let mut active = user.into_active_model();
        if let Some(description) = parma.description {
            active.description = sea_orm::ActiveValue::Set(Option::from(description));
        }
        if let Some(website) = parma.website {
            active.website = sea_orm::ActiveValue::Set(Option::from(website));
        }
        if let Some(location) = parma.location {
            active.location = sea_orm::ActiveValue::Set(Option::from(location));
        }
        if let Some(timezone) = parma.timezone {
            active.timezone = sea_orm::ActiveValue::Set(Option::from(timezone));
        }
        if let Some(language) = parma.language {
            active.language = sea_orm::ActiveValue::Set(Option::from(language));
        }
        active.updated_at = sea_orm::ActiveValue::Set(chrono::Local::now().naive_local());
        active.update(&self.write).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
    pub async fn user_avatar_update(&self, uid: Uuid, avatar: String) -> io::Result<users::Model> {
        let user = self.user_info_by_uid(uid).await?;
        let mut active = user.into_active_model();
        active.avatar = sea_orm::ActiveValue::Set(Option::from(avatar));
        active.updated_at = sea_orm::ActiveValue::Set(chrono::Local::now().naive_local());
        active.update(&self.write).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}