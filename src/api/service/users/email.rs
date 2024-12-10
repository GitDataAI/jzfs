use sea_orm::*;
use uuid::Uuid;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users_email;

impl UserService {
    pub async fn email(&self, uid: Uuid) -> anyhow::Result<Vec<users_email::Model>> {
        anyhow::Ok(users_email::Entity::find()
            .filter(
                users_email::Column::UserId.eq(uid)
            )
            .all(&self.db)
            .await?
        )
    }
}