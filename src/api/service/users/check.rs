use sea_orm::*;
use crate::api::service::users::UserService;
use crate::metadata::model::users::{users, users_email};

impl UserService {
    pub async fn check_username(&self, username: String) -> anyhow::Result<bool>{
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.db)
            .await?;
        match user {
            Some(_) => Ok(false),
            None => Ok(true)
        }
    }
    pub async fn check_email(&self, email: String) -> anyhow::Result<bool>{
        let user = users_email::Entity::find()
            .filter(users_email::Column::Email.eq(email))
            .one(&self.db)
            .await?;
        match user {
            Some(_) => Ok(false),
            None => Ok(true)
        }
    }
}