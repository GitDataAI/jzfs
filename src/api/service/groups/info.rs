use sea_orm::*;
use uuid::Uuid;
use crate::api::service::groups::GroupService;
use crate::metadata::model::groups::group;
use crate::metadata::model::teams::teams;

impl GroupService {
    pub async fn info(&self, uid: Uuid) -> anyhow::Result<group::Model>{
        match group::Entity::find_by_id(uid).one(&self.db).await?{
            Some(x) => Ok(x),
            None => Err(anyhow::anyhow!("[Error] Group Not Found"))
        }
    }
    pub async fn infos(&self, uids: Vec<Uuid>) -> anyhow::Result<Vec<group::Model>>{
        Ok(group::Entity::find().filter(group::Column::Uid.is_in(uids)).all(&self.db).await?)
    }
    pub async fn team(&self, uid: Uuid) -> anyhow::Result<Vec<Uuid>>{
        let teams = teams::Entity::find()
            .filter(
                teams::Column::GroupId.eq(uid)
            )
            .all(&self.db)
            .await?;
        let teams = teams.into_iter().map(|x| x.uid).collect::<Vec<_>>();
        Ok(teams)
    }
}