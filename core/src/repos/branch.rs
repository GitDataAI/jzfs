use crate::AppCore;
use anyhow::anyhow;
use database::entity::{git_commit, git_refs, user_repo};
use error::AppError;
use git::GitContext;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, IntoActiveModel, TransactionTrait};
use serde_json::json;
use session::Session;

impl AppCore {
    pub async fn repos_branch_list(
        &self,
        namespace: &str,
        repo_name: &str,
    ) -> Result<serde_json::Value, AppError> {
        let repo = self.repo_find(namespace, repo_name).await?;
        let refs = git_refs::Entity::find()
            .filter(git_refs::Column::RepoUid.eq(repo.uid))
            .all(&self.db)
            .await?;

        let mut result = vec![];
        for ref_item in refs {
            let head = if let Some(head) = git_commit::Entity::find()
                .filter(
                    Condition::all()
                        .add(git_commit::Column::RepoUid.eq(repo.uid))
                        .add(git_commit::Column::RefsUid.eq(ref_item.uid))
                        .add(git_commit::Column::CommitId.eq(ref_item.ref_git_id.clone())),
                )
                .one(&self.db)
                .await?
            {
                json!(head)
            } else {
                json!(ref_item.ref_git_id)
            };
            let item = json!({
                "branch": ref_item,
                "head": head
            });
            result.push(item);
        }
        Ok(json!(result))
    }
    pub async fn repo_branch_delete(
        &self,
        namespace: &str,
        repo_name: &str,
        branch_name: &str,
        session: Session,
    ) -> Result<(), AppError> {
        let repo = self.repo_find(namespace, repo_name).await?;
        let user = self.user_context(session).await?;
        if let Ok(relate) = user_repo::Entity::find()
            .filter(
                Condition::all()
                    .add(user_repo::Column::RepoUid.eq(repo.uid))
                    .add(user_repo::Column::UserUid.eq(user.user_uid)),
            )
            .all(&self.db)
            .await
        {
            if relate.len() <= 0 {
                return Err(AppError::from(anyhow!("permission denied")));
            }
        } else {
            return Err(AppError::from(anyhow!("permission denied")));
        }
        let txn = self.db.begin().await?;
        let branch = git_refs::Entity::find()
            .filter(
                Condition::all()
                    .add(git_refs::Column::RepoUid.eq(repo.uid))
                    .add(git_refs::Column::RefName.eq(branch_name)),
            )
            .one(&txn)
            .await?
            .ok_or(AppError::from(anyhow!("branch not found")))?;
        let branch_active = branch.clone().into_active_model();
        branch_active.delete(&txn).await?;
        let commits = git_commit::Entity::find()
            .filter(
                Condition::all()
                    .add(git_commit::Column::RepoUid.eq(repo.uid))
                    .add(git_commit::Column::RefsUid.eq(branch.uid)),
            )
            .all(&txn)
            .await?;
        for commit in commits {
            let commit_active = commit.into_active_model();
            commit_active.delete(&txn).await?;
        }
        let git = GitContext::try_from((repo, self.config.git.clone()))?;
        git.refs_delete(branch_name)?;
        txn.commit().await?;
        Ok(())
    }
}
