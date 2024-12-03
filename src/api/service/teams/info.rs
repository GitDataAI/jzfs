use sea_orm::*;
use crate::api::service::teams::TeamService;
use crate::metadata::model::teams::teams;

impl TeamService {
    pub async fn info(&self, uid: uuid::Uuid) -> anyhow::Result<teams::Model>{
        match teams::Entity::find_by_id(uid).one(&self.db).await?{
            Some(x) => Ok(x),
            None => Err(anyhow::anyhow!("[Error] Team Not Found"))
        }
    }
}