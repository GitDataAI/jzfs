use uuid::Uuid;
use crate::metadata::model::repo::repo;
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
impl RepoService {
    pub async fn owner_name_by_uid(&self, owner: String, name: String) -> anyhow::Result<Uuid>{
        let model = repo::Entity::find()
            .filter(repo::Column::Name.eq(name))
            .filter(repo::Column::Owner.eq(owner))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        Ok(model.unwrap().uid)
    }
}