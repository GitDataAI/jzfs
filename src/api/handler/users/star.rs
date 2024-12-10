use sea_orm::*;
use uuid::Uuid;
use crate::api::ov::repos::RepoBasicInfo;
use crate::api::service::users::UserService;
use crate::metadata::model::repos::repo;
use crate::metadata::model::users::users_other;

impl UserService {
    pub async fn star(&self, uid: Uuid) -> anyhow::Result<Vec<RepoBasicInfo>> {
        let repo_ids = users_other::Entity::find()
            .filter(
                users_other::Column::UserId.eq(uid)
            )
            .one(&self.db)
            .await?;
        if repo_ids.is_none(){
            return Err(anyhow::anyhow!("[Error] User Not Exist"))
        }
        let repo_ids = repo_ids.unwrap().star;
        let models = repo::Entity::find()
            .filter(
                repo::Column::Uid.is_in(repo_ids)
            )
            .all(&self.db)
            .await?;
        Ok(models.into_iter().map(|x| x.into()).collect::<Vec<_>>())
    }
}