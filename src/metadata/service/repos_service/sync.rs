use std::collections::HashMap;
use crate::git::branchs::GitBranch;
use crate::git::repo::GitRepo;
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::metadata::model::repo::{repo_branch, repo_commit};

impl RepoService {
    pub async fn sync_repo<'a>(&self, owner: String, repo: String, owner_id: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await?;

        let uid = self.owner_name_by_uid(owner.clone(),repo.clone()).await;
        if uid.is_err(){
            return Err(uid.err().unwrap())
        }
        let repo_uid = uid?;

        let models = repo_branch::Entity::find()
            .filter(repo_branch::Column::RepoId.eq(repo_uid))
            .all(&self.db)
            .await?;
        for model in models{
            model.into_active_model().delete(&self.db).await?;
        }
        let models = repo_commit::Entity::find()
            .filter(repo_commit::Column::RepoId.eq(repo_uid))
            .all(&self.db)
            .await?;
        for model in models{
            model.into_active_model().delete(&self.db).await?;
        }

        let model = self.info(repo_uid).await;
        if model.is_err(){
            return Err(model.err().unwrap())
        }
        let model = model?;
        let repo = GitRepo::from(model);
        let branchs = GitBranch::new(repo.repo);
        let branchs = branchs.branchs();
        if branchs.is_err() {
            return Err(branchs.err().unwrap());
        }
        let mut map = HashMap::new();
        let branchs = branchs?;
        for branchs_idx in branchs {
            let branch = branchs_idx.name()?.unwrap().to_string();
            let refs = branchs_idx.into_reference();
            let peel_commit = refs.peel_to_commit();
            let mut commits = Vec::new();
            let branch_uid = Uuid::new_v4();
            if let Ok(commit) = peel_commit {
                let mut current_commit = Some(commit);
                while let Some(commit) = current_commit {
                    let commit_id = commit.id().to_string();
                    let commit_time = OffsetDateTime::from_unix_timestamp(commit.time().seconds())?;
                    let commit_bio = commit.message().unwrap_or("").to_string();
                    let (commit_username, commit_email) = {
                        if commit.author().name().is_some() {
                            (
                                commit.author().name().unwrap_or("").to_string(),
                                commit.author().email().unwrap_or("").to_string(),
                            )
                        } else {
                            ("".to_string(), "".to_string())
                        }
                    };
                    let commit_model = repo_commit::ActiveModel {
                        uid: Set(Uuid::new_v4()),
                        repo_id: Set(repo_uid),
                        branch_id: Set(branch_uid),
                        bio: Set(commit_bio),
                        commit_user: Set(commit_username),
                        commit_email: Set(commit_email),
                        commit_id: Set(commit_id),
                        created_at: Set(commit_time),
                    };
                    commits.push(commit_model);

                    // 移动到下一个父提交
                    current_commit = commit.parent(0).ok();
                }
            } else {
                continue;
            }
            let branch_model = repo_branch::ActiveModel {
                uid: Set(branch_uid),
                repo_id: Set(repo_uid),
                branch: Set(branch),
                head: Set({
                    if commits.first().is_some(){
                        Option::from(commits.first().unwrap().clone().uid.unwrap())
                    }else{
                        Option::from(None)
                    }
                }),
                visible: Set(true),
                protect: Set(false),
                created_at: Set({
                    if commits.last().is_some(){
                        commits.last().unwrap().clone().created_at.unwrap()
                    }else{
                        OffsetDateTime::now_utc()
                    }
                }),
                updated_at: Set({
                    if commits.last().is_some(){
                        commits.last().unwrap().clone().created_at.unwrap()
                    }else{
                        OffsetDateTime::now_utc()
                    }
                }),
                created_by: Set(owner_id),
            };
            map.insert(branch_model, commits);
        }
        for (branch_model,commits) in map{
            for commit in commits{
                commit.insert(&txn).await?;
            }
            branch_model.insert(&txn).await?;
        }
        txn.commit().await?;
        Ok(())
    }
}