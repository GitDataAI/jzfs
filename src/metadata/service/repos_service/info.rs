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
    pub async fn search(&self, keyword: String, page: u64, size: u64) -> anyhow::Result<Vec<repo::Model>>{
        let models = repo::Entity::find()
            .filter(repo::Column::Name.contains(keyword.clone()))
            .filter(repo::Column::Description.contains(keyword.clone()))
            .filter(repo::Column::Topic.contains(keyword))
            .filter(repo::Column::Visible.eq(true))
            .offset(page * size)
            .limit(size)
            .all(&self.db)
            .await?;
        Ok(models)
    }
    pub async fn info(&self, uid: Uuid) -> anyhow::Result<repo::Model>{
        let model = repo::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        Ok(model.unwrap())
    }
    
}