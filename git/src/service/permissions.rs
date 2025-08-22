use crate::service::GitServer;
use database::entity::{git_repo, user_repo, users};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

impl GitServer {
    pub async fn ssh_permissions(&self, repo: git_repo::Model, user: users::Model) -> bool {
        if let Ok(list) = user_repo::Entity::find()
            .filter(user_repo::Column::UserUid.eq(user.uid))
            .all(&self.db)
            .await
        {
            for item in list {
                if item.repo_uid == repo.uid {
                    return true;
                }
            }
        }
        false
    }
}
