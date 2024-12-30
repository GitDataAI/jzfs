use crate::error::{JZError, JZResult};
use crate::models::repos::{branchs, commits, repos};
use crate::server::MetaData;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn repo_commits(
        &self,
        owner: String,
        name: String,
        branchs: String,
        page: u64,
        size: u64,
    ) -> JZResult<Vec<commits::Model>> {
        let result = repos::Entity::find()
            .filter(repos::Column::Owner.eq(owner))
            .filter(repos::Column::Name.eq(name))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[034] Repo Not Found")));
        }
        let result = branchs::Entity::find()
            .filter(branchs::Column::RepoId.eq(result.unwrap().uid))
            .filter(branchs::Column::Name.eq(branchs))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[035] Branchs Not Found")));
        }
        let result = commits::Entity::find()
            .filter(commits::Column::BranchId.eq(result.unwrap().uid))
            .order_by_desc(commits::Column::Created)
            .paginate(&self.database, size)
            .fetch_page(page)
            .await?;
        Ok(result)
    }
    pub async fn repo_commit_sha(
        &self,
        owner: String,
        name: String,
        branchs: String,
        sha: String,
    ) -> JZResult<commits::Model> {
        let result = repos::Entity::find()
            .filter(repos::Column::Owner.eq(owner))
            .filter(repos::Column::Name.eq(name))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[036] Repo NotFound")));
        }
        let result = branchs::Entity::find()
            .filter(branchs::Column::RepoId.eq(result.unwrap().uid))
            .filter(branchs::Column::Name.eq(branchs))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[037] Branch Not Found")));
        }
        let result = commits::Entity::find()
            .filter(
                Condition::all()
                    .add(commits::Column::BranchId.eq(result.unwrap().uid))
                    .add(commits::Column::CommitId.eq(sha)),
            )
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[038] Commit Not Found")));
        }
        Ok(result.unwrap())
    }
    pub async fn repo_commits_sync(&self, repo_id: Uuid, branch: String) -> JZResult<()> {
        match repos::Entity::find_by_id(repo_id)
            .one(&self.database)
            .await?
        {
            Some(model) => model,
            None => return Err(JZError::Other(anyhow::anyhow!("[045] Repo NotFound "))),
        };
        let branch = match branchs::Entity::find()
            .filter(
                Condition::all()
                    .add(branchs::Column::RepoId.eq(repo_id))
                    .add(branchs::Column::Name.eq(branch)),
            )
            .one(&self.database)
            .await?
        {
            Some(model) => model,
            None => return Err(JZError::Other(anyhow::anyhow!("[046] Branch NotFound "))),
        };
        let local_repo = match self.git.open_repo(repo_id.to_string()) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[047] Open Repo Failed: {}",
                    e.to_string()
                )))
            }
        };
        let txn = self.database.begin().await?;
        let commits = local_repo.commit_list()?;
        for commit in commits {
            if branch.name != commit.branchs {
                continue;
            }
            let commit_id = commit.hash_oid;
            if commits::Entity::find()
                .filter(
                    Condition::all()
                        .add(commits::Column::RepoId.eq(repo_id))
                        .add(commits::Column::BranchId.eq(branch.uid))
                        .add(commits::Column::CommitId.eq(commit_id.clone())),
                )
                .one(&self.database)
                .await?
                .is_some()
            {
                continue;
            }
            let _ = commits::ActiveModel {
                uid: sea_orm::ActiveValue::Set(Uuid::new_v4()),
                repo_id: sea_orm::ActiveValue::Set(repo_id),
                branch_id: sea_orm::ActiveValue::Set(branch.uid),
                description: sea_orm::ActiveValue::Set(commit.msg),
                commit_user: sea_orm::ActiveValue::Set(commit.username),
                commit_email: sea_orm::ActiveValue::Set(commit.email),
                commit_id: sea_orm::ActiveValue::Set(commit_id),
                created: sea_orm::ActiveValue::Set(commit.time),
            }
            .insert(&txn)
            .await;
        }
        txn.commit().await?;
        Ok(())
    }
}
