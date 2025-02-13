use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::prelude::Expr;
use lib_entity::repos::repos;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::service::AppFsState;

#[derive(Deserialize, Serialize, Clone)]
pub struct AvatarUploadParma {
    pub repository : Uuid,
    pub users_uid : Uuid,
    pub avatar_url : String,
}

impl AppFsState {
    pub async fn avatar_update(
        &self,
        repository : Uuid,
        users_uid : Uuid,
        avatar_url : String,
    ) -> anyhow::Result<()> {
        if !self.access(repository, users_uid).await? {
            return Err(anyhow::anyhow!("no access"));
        }
        repos::Entity::update_many()
            .col_expr(repos::Column::AvatarUrl, Expr::value(avatar_url))
            .filter(repos::Column::Uid.eq(repository))
            .exec(&self.write)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(())
    }
    pub async fn avatar_delete(&self, repository : Uuid, users_uid : Uuid) -> anyhow::Result<()> {
        if !self.access(repository, users_uid).await? {
            return Err(anyhow::anyhow!("no access"));
        }
        repos::Entity::update_many()
            .col_expr(repos::Column::AvatarUrl, Expr::value(""))
            .filter(repos::Column::Uid.eq(repository))
            .exec(&self.write)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(())
    }
}
