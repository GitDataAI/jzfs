use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::PaginatorTrait;
use lib_entity::QueryFilter;
use lib_entity::QueryOrder;
use lib_entity::repos::commits;
use uuid::Uuid;

use crate::service::AppFsState;

impl AppFsState {
    pub async fn list_commit(
        &self,
        repos_uid : Uuid,
        branch_uid : Uuid,
        page : u64,
        page_size : u64,
    ) -> anyhow::Result<Vec<commits::Model>> {
        commits::Entity::find()
            .filter(commits::Column::RepoId.eq(repos_uid))
            .filter(commits::Column::BranchId.eq(branch_uid))
            .order_by_desc(commits::Column::Created)
            .paginate(&self.read, page_size)
            .fetch_page(page)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
    pub async fn status_commit(
        &self,
        repos_uid : Uuid,
        commits_hash : String,
    ) -> anyhow::Result<commits::Model> {
        commits::Entity::find()
            .filter(commits::Column::RepoId.eq(repos_uid))
            .filter(commits::Column::CommitId.eq(commits_hash))
            .one(&self.read)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))
            .and_then(|r| match r {
                Some(r) => Ok(r),
                None => Err(anyhow::anyhow!("Commit not found")),
            })
    }
}
