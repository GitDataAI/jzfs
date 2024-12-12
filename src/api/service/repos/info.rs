use anyhow::anyhow;
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
    pub async fn rename(&self, uid: Uuid, name: String) -> anyhow::Result<repo::Model>{
        let repo = self.info(uid).await?;
        let mut model = repo.into_active_model();
        model.name = Set(name);
        match model.update(&self.db).await {
            Ok(model) => {
                Ok(model)
            },
            Err(e) => {
                Err(anyhow!(e))
            }
        }
    }
    pub async fn topics(&self, uid: Uuid) -> anyhow::Result<Vec<String>>{
        let repo = self.info(uid).await?;
        Ok(repo.topic)
    }
    pub async fn add_topic(&self, uid: Uuid, topic: String) -> anyhow::Result<repo::Model>{
        let repo = self.info(uid).await?;
        let mut topics = repo.topic.clone();
        if !topics.contains(&topic){
            topics.push(topic);
        }
        let mut model = repo.into_active_model();
        model.topic = Set(topics);
        match model.update(&self.db).await {
            Ok(model) => {
                Ok(model)
            },
            Err(e) => {
                Err(anyhow!(e))
            }
        }
    }
    pub async fn del_topic(&self, uid: Uuid, topic: String) -> anyhow::Result<repo::Model>{
        let mut repo = self.info(uid).await?;
        let mut topics = repo.topic.clone();
        if topics.contains(&topic){
            topics.retain(|x| x != &topic);
        }
        let mut model = repo.into_active_model();
        model.topic = Set(topics);
        match model.update(&self.db).await {
            Ok(model) => {
                Ok(model)
            },
            Err(e) => {
                Err(anyhow!(e))
            }
        }
    }
}