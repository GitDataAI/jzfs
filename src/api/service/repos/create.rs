use uuid::Uuid;
use crate::api::dto::repo::RepoCreate;
use crate::api::service::repos::RepoService;

impl RepoService {
    pub async fn create_repo(&self, dto: RepoCreate, created_by: Uuid) -> anyhow::Result<()>{
        self.transaction.create_repo(dto, created_by).await?;
        Ok(())
    }
}