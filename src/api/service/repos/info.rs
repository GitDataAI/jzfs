use sea_orm::*;
use uuid::Uuid;
use crate::api::service::repos::RepoService;
use crate::metadata::model::repos::repo;

impl RepoService {
    pub async fn info(&self, uid: Uuid) -> anyhow::Result<repo::Model>{
        let model = repo::Entity::find_by_id(uid).one(&self.db).await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("Repo Not Found"))
        }
        Ok(model.unwrap())
    }
}