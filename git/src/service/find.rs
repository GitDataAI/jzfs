use crate::service::GitServer;
use anyhow::anyhow;
use database::entity::{git_repo, ssh_keys, user_access_keys, users};
use error::AppError;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::prelude::Uuid;
use sea_orm::{Condition, EntityTrait};

impl GitServer {
    pub async fn find_repo(
        &self,
        namespace: &str,
        repo_name: &str,
    ) -> Result<git_repo::Model, AppError> {
        let repo = git_repo::Entity::find()
            .filter(
                Condition::all()
                    .add(git_repo::Column::Namespace.eq(namespace))
                    .add(git_repo::Column::RepoName.eq(repo_name)),
            )
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Repo not found")))?;
        Ok(repo)
    }
    pub async fn find_repo_owner(&self, repo: git_repo::Model) -> Result<users::Model, AppError> {
        let owner = users::Entity::find()
            .filter(Condition::all().add(users::Column::Username.eq(repo.namespace)))
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Repo owner not found")))?;
        // TODO team
        Ok(owner)
    }
    pub async fn find_repo_by_id(&self, repo_id: Uuid) -> Result<git_repo::Model, AppError> {
        let repo = git_repo::Entity::find_by_id(repo_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Repo not found")))?;
        Ok(repo)
    }
    pub async fn find_ssh_key_owner(&self, ssh_key: &str) -> Result<users::Model, AppError> {
        let owner = ssh_keys::Entity::find()
            .filter(Condition::all().add(ssh_keys::Column::Content.eq(ssh_key)))
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("SSH key not found")))?;
        let users = users::Entity::find()
            .filter(Condition::all().add(users::Column::Uid.eq(owner.user_id)))
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("SSH key owner not found")))?;
        Ok(users)
    }
    pub async fn find_token_owner(
        &self,
        token: &str,
        need_access: i32,
    ) -> Result<users::Model, AppError> {
        let token = user_access_keys::Entity::find()
            .filter(Condition::all().add(user_access_keys::Column::Token.eq(token)))
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Token not found")))?;
        if token.repo_access < need_access {
            return Err(AppError::from(anyhow!(
                "The access key permission is insufficient and the operation cannot be completed."
            )));
        }
        let users = users::Entity::find()
            .filter(Condition::all().add(users::Column::Uid.eq(token.resource_owner_uid)))
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Token owner not found")))?;
        Ok(users)
    }
}
