use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::repos::repos;
use lib_entity::users::users;
use uuid::Uuid;

use crate::service::AppFsState;

impl AppFsState {
    pub async fn access(&self, users_uid : Uuid, repo_id : Uuid) -> anyhow::Result<bool> {
        let repo = repos::Entity::find()
            .filter(repos::Column::Uid.eq(repo_id))
            .one(&self.read)
            .await
            .map_err(|_| anyhow::anyhow!("repo not found"))?
            .ok_or(anyhow::anyhow!("repo not found"))?;
        if repo.owner_id == users_uid {
            return Ok(true);
        }
        let owner = users::Entity::find()
            .filter(users::Column::Uid.eq(repo.owner_id))
            .all(&self.read)
            .await
            .map_err(|_| anyhow::anyhow!("repo not found"))?;
        if owner.iter().any(|x| x.member.contains(&users_uid)) {
            return Ok(true);
        }
        Ok(false)
    }
}
