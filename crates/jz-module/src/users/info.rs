use crate::AppModule;
use jz_model::users;
use sea_orm::*;
use uuid::Uuid;

impl AppModule {
    pub async fn user_info_by_id(&self, uid: Uuid) -> anyhow::Result<users::Model> {
        users::Entity::find_by_id(uid)
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("user not found"))
    }
    pub async fn user_info_by_email(&self, email: String) -> anyhow::Result<users::Model> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("user not found"))
    }
    pub async fn user_info_by_username(&self, username: String) -> anyhow::Result<users::Model> {
        users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("user not found"))
    }
}
