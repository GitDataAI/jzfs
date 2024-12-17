use crate::metadata::model::users::users_data;
use crate::metadata::service::users_service::UserService;
use sea_orm::*;
use crate::metadata::model::repo::repo;

impl UserService {
    pub async fn owner_repo(&self, uid: uuid::Uuid) -> anyhow::Result<Vec<repo::Model>> {
        let model = users_data::Entity::find()
            .filter(
                users_data::Column::UserId.eq(uid)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        if model.repo.is_empty(){
            return Ok(vec![]);
        }
        let models = repo::Entity::find()
            .filter(
                repo::Column::Uid.is_in(model.repo)
            )
            .all(&self.db)
            .await?;
        Ok(models)
    }
}