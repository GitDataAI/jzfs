use crate::error::{JZError, JZResult};
use crate::models::repos::repos;
use crate::models::users::{email, follower, star, users};
use crate::server::MetaData;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn check_user_username(&self, username: String) -> JZResult<bool> {
        let models = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.database)
            .await?;
        Ok(models.is_some())
    }
    pub async fn check_user_email(&self, email: String) -> JZResult<bool> {
        let models = users::Entity::find()
            .filter(users::Column::MainEmail.eq(email.clone()))
            .one(&self.database)
            .await?;
        if models.is_none() {
            return Ok(false);
        }
        let models = email::Entity::find()
            .filter(email::Column::Content.eq(email))
            .one(&self.database)
            .await?;
        Ok(models.is_some())
    }
    pub async fn check_users_id(&self, uid: Uuid) -> JZResult<bool> {
        let models = users::Entity::find_by_id(uid).one(&self.database).await?;
        Ok(models.is_some())
    }
    pub async fn check_users_follower(&self, uid: Uuid, follower: Uuid) -> JZResult<bool> {
        let models = users::Entity::find_by_id(uid).one(&self.database).await?;
        if models.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[025] User Not Found")));
        }
        let models = users::Entity::find_by_id(follower)
            .one(&self.database)
            .await?;
        if models.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[026] User Not Fund")));
        }
        let models = follower::Entity::find()
            .filter(follower::Column::UserId.eq(uid))
            .filter(follower::Column::FollowerId.eq(follower))
            .one(&self.database)
            .await?;
        Ok(models.is_some())
    }
    pub async fn check_repo_id(&self, rid: Uuid) -> JZResult<bool> {
        let models = repos::Entity::find_by_id(rid).one(&self.database).await?;
        Ok(models.is_some())
    }
    pub async fn check_users_star(&self, uid: Uuid, rid: Uuid) -> JZResult<bool> {
        let models = star::Entity::find()
            .filter(star::Column::UserId.eq(uid))
            .filter(star::Column::RepoId.eq(rid))
            .one(&self.database)
            .await?;
        Ok(models.is_some())
    }
}
