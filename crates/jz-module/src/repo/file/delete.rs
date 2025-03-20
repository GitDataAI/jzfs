use crate::AppModule;
use jz_model::{branch, commit, repository, star, watch};
use sea_orm::*;
use uuid::Uuid;

impl AppModule {
    pub async fn repo_delete(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let user = self.user_info_by_id(ops_uid).await?;
        if repo.owner_uid != user.uid {
            return Err(anyhow::anyhow!("no permission"));
        }
        // todo GROUP
        let txn = self.write.begin().await?;

        star::Entity::delete_many()
            .filter(star::Column::RepositoryId.eq(repo.uid))
            .exec(&txn)
            .await?;
        repository::Entity::delete_many()
            .filter(repository::Column::Uid.eq(repo.uid))
            .exec(&txn)
            .await?;
        watch::Entity::delete_many()
            .filter(watch::Column::RepoUid.eq(repo.uid))
            .exec(&txn)
            .await?;
        commit::Entity::delete_many()
            .filter(commit::Column::RepoUid.eq(repo.uid))
            .exec(&txn)
            .await?;
        branch::Entity::delete_many()
            .filter(branch::Column::RepoUid.eq(repo.uid))
            .exec(&txn)
            .await?;
        txn.commit().await?;
        let git = repo.git()?;
        git.delete()?;
        Ok(())
    }
}
