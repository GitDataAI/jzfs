use sea_orm::*;
use uuid::Uuid;
use crate::metadata::model::repo::repo_license;
use crate::metadata::service::repos_service::RepoService;

impl RepoService {
    pub async fn licenses(&self, repo_id: Uuid) -> anyhow::Result<Vec<repo_license::Model>>{
        let licenses = repo_license::Entity::find()
            .filter(repo_license::Column::RepoId.eq(repo_id))
            .all(&self.db)
            .await?;
        Ok(licenses)
    }
}