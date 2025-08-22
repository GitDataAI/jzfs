use crate::AppCore;
use anyhow::anyhow;
use database::entity::{user_repo_active, users};
use error::AppError;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{Condition, EntityTrait};

impl AppCore {
    pub async fn users_active(
        &self,
        username: &str,
    ) -> Result<Vec<user_repo_active::Model>, AppError> {
        let user = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(username))
                    .add(users::Column::DisplayName.eq(username)),
            )
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("user not found")))?;
        let active = user_repo_active::Entity::find()
            .filter(Condition::any().add(user_repo_active::Column::UserUid.eq(user.uid)))
            .all(&self.db)
            .await?;
        Ok(active)
    }
}
