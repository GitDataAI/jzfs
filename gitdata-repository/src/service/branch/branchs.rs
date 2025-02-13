use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::repos::branchs;
use lib_entity::repos::commits;
use lib_entity::repos::repos;
use serde::Serialize;
use uuid::Uuid;

use crate::service::AppFsState;

#[derive(Serialize, Clone)]
pub struct BranchesInfo {
    pub uid : Uuid,
    pub repo_id : Uuid,
    pub name : String,
    pub head : Option<String>,
    pub protect : bool,
    pub commit : Vec<commits::Model>,
}

impl AppFsState {
    pub async fn list_branch(&self, repos_uid : Uuid) -> anyhow::Result<Vec<BranchesInfo>> {
        let repo = repos::Entity::find()
            .filter(repos::Column::Uid.eq(repos_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("repos not found"))?;
        let branches = branchs::Entity::find()
            .filter(branchs::Column::RepoId.eq(repo.uid))
            .all(&self.read)
            .await?;
        let mut result = Vec::new();
        for idx in branches {
            let commits = commits::Entity::find()
                .filter(commits::Column::BranchId.eq(idx.uid))
                .all(&self.read)
                .await
                .unwrap_or_else(|_| Vec::new());
            result.push(BranchesInfo {
                uid : idx.uid,
                repo_id : idx.repo_id,
                name : idx.name,
                head : idx.head,
                protect : idx.protect,
                commit : commits,
            });
        }
        Ok(result)
    }
    pub async fn create_branch(
        &self,
        repos_uid : Uuid,
        branch_name : String,
    ) -> anyhow::Result<()> {
        let repo = repos::Entity::find()
            .filter(repos::Column::Uid.eq(repos_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("repos not found"))?;
        // todo
        Ok(())
    }
    pub async fn delete_branch(
        &self,
        repos_uid : Uuid,
        branch_name : String,
    ) -> anyhow::Result<()> {
        let repo = repos::Entity::find()
            .filter(repos::Column::Uid.eq(repos_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("repos not found"))?;
        // todo
        Ok(())
    }
}
