use uuid::Uuid;
use crate::metadata::model::groups::groups_data;
use crate::metadata::model::repo::repo;
use sea_orm::*;
use crate::metadata::service::groups_service::GroupService;

impl GroupService {
    pub async fn owner_repo(&self, uid: Uuid) -> anyhow::Result<Vec<repo::Model>>{
        let data = groups_data::Entity::find()
            .filter(groups_data::Column::GroupId.eq(uid))
            .all(&self.db)
            .await?;
        let repo_ids = data.iter().map(|x| x.repo_id).collect::<Vec<_>>();
        let models = repo::Entity::find()
            .filter(repo::Column::Uid.is_in(repo_ids))
            .all(&self.db)
            .await?;
        Ok(models)
    }
}