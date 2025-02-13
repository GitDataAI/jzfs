use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::repos::repos;
use lib_entity::users::users;
use uuid::Uuid;

use crate::service::AppFsState;

impl AppFsState {
    pub async fn delete_repo(&self, users_uid : Uuid, repo_id : Uuid) -> anyhow::Result<()> {
        let repo = repos::Entity::find()
            .filter(repos::Column::Uid.eq(repo_id))
            .one(&self.read)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .ok_or_else(|| anyhow::anyhow!("repository not found"))?;
        if repo.owner_id != users_uid {
            if users::Entity::find()
                .filter(users::Column::Member.contains(users_uid))
                .one(&self.read)
                .await
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .ok_or_else(|| anyhow::anyhow!("repository not found"))
                .is_err()
            {
                return Err(anyhow::anyhow!("permission denied"));
            }
        }
        // TODO
        Ok(())
    }
}
