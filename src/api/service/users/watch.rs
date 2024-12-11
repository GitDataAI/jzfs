use sea_orm::*;
use uuid::Uuid;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users_other;

impl UserService {
    pub async fn wacther(&self, uid: Uuid) -> anyhow::Result<Vec<Uuid>>{
        let model = users_other::Entity::find()
            .filter(users_other::Column::Uid.eq(uid))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Exist"))
        }
        let model = model.unwrap();
        Ok(model.watcher)
    }
}