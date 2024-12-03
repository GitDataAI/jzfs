use sea_orm::*;
use uuid::Uuid;
use crate::api::service::groups::GroupService;
use crate::metadata::model::groups::group;

impl GroupService {
    pub async fn info(&self, uid: Uuid) -> anyhow::Result<group::Model>{
        match group::Entity::find_by_id(uid).one(&self.db).await?{
            Some(x) => Ok(x),
            None => Err(anyhow::anyhow!("[Error] Group Not Found"))
        }
    }
}