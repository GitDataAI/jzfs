use sea_orm::*;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users;

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
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await?;
        match user {
            Some(_) => Ok(false),
            None => Ok(true)
        }
    }
}