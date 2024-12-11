use sea_orm::*;
use crate::api::service::repos::RepoService;
use crate::metadata::model::groups::group_repo;
use crate::metadata::model::repos::repo;
use crate::metadata::model::repos::repo::Model;
use crate::metadata::model::users::users_other;

impl RepoService {
    pub async fn owner(&self, uid: uuid::Uuid) -> anyhow::Result<Vec<Model>> {
        let model = users_other::Entity::find()
            .filter(
                users_other::Column::UserId.eq(uid)
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
    pub async fn repo_by_group(&self, group_id: uuid::Uuid) -> anyhow::Result<Vec<Model>>{
        let models = group_repo::Entity::find()
            .filter(
                group_repo::Column::GroupId.eq(group_id)
            )
            .all(
                &self.db
            )
            .await?;
        let models = models.into_iter().map(|x| x.repo_id).collect::<Vec<uuid::Uuid>>();
        let mut result = vec![];
        for model in models{
            let model = repo::Entity::find_by_id(model).one(&self.db).await?;
            if model.is_none(){
                continue;
            }
            result.push(model.unwrap());
        }
        Ok(result)
    }
}