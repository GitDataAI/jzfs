use crate::{AppCore, Paginator};
use anyhow::anyhow;
use database::entity::{git_commit, git_refs, user_repo_active};
use error::AppError;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde_json::json;

impl AppCore {
    pub async fn repos_commit_list(
        &self,
        namespace: &str,
        repo_name: &str,
        branch: &str,
        paginator: Paginator,
    ) -> Result<serde_json::Value, AppError> {
        let repo = self.repo_find(namespace, repo_name).await?;
        let branch = git_refs::Entity::find()
            .filter(git_refs::Column::RepoUid.eq(repo.uid))
            .all(&self.db)
            .await?
            .into_iter()
            .find(|x| x.ref_name == branch)
            .ok_or(AppError::from(anyhow!("Branch not found")))?;
        let commits = git_commit::Entity::find()
            .filter(
                Condition::all()
                    .add(git_commit::Column::RepoUid.eq(repo.uid))
                    .add(git_commit::Column::RefsUid.eq(branch.uid)),
            )
            .order_by_desc(git_commit::Column::Time)
            .limit(paginator.page_size)
            .offset(paginator.page_size * paginator.page)
            .all(&self.db)
            .await?;
        let mut values = vec![];
        for commit in commits {
            let commiter_value = if let Some(commiter) = commit.committer {
                let commiter = user_repo_active::Entity::find()
                    .filter(
                        Condition::all()
                            .add(user_repo_active::Column::Uid.eq(commiter))
                            .add(user_repo_active::Column::RepoUid.eq(repo.uid))
                            .add(user_repo_active::Column::Commit.eq(commit.uid)),
                    )
                    .one(&self.db)
                    .await?;
                if let Some(commiter) = commiter {
                    if let Some(user_uid) = commiter.user_uid {
                        if let Ok(user) = self.user_context_find_by_uid(user_uid).await {
                            json!({
                                "uid": commiter.uid,
                                "name": user.username,
                                "email": user.email,
                                "avatar": user.avatar_url,
                                "time": commiter.time,
                                "offset": commiter.offset,
                                "avatar": user.avatar_url,
                                "repo_uid": commiter.repo_uid
                            })
                        } else {
                            json!(commiter)
                        }
                    } else {
                        json!(commiter)
                    }
                } else {
                    json!(commiter)
                }
            } else {
                serde_json::Value::Null
            };
            let author_value = if let Some(author) = commit.author {
                let commiter = user_repo_active::Entity::find()
                    .filter(
                        Condition::all()
                            .add(user_repo_active::Column::Uid.eq(author))
                            .add(user_repo_active::Column::RepoUid.eq(repo.uid))
                            .add(user_repo_active::Column::Commit.eq(commit.uid)),
                    )
                    .one(&self.db)
                    .await?;
                if let Some(commiter) = commiter {
                    if let Some(user_uid) = commiter.user_uid {
                        if let Ok(user) = self.user_context_find_by_uid(user_uid).await {
                            json!({
                                "uid": commiter.uid,
                                "name": user.username,
                                "email": user.email,
                                "avatar": user.avatar_url,
                                "time": commiter.time,
                                "offset": commiter.offset,
                                "avatar": user.avatar_url,
                                "repo_uid": commiter.repo_uid
                            })
                        } else {
                            json!(commiter)
                        }
                    } else {
                        json!(commiter)
                    }
                } else {
                    json!(commiter)
                }
            } else {
                serde_json::Value::Null
            };

            values.push(json!({
                "uid": commit.uid,
                "message": commit.content,
                "commiter": commiter_value,
                "time": commit.time,
                "author": author_value,
                "parents": commit.parents_id,
                "tree": commit.tree,
                "commit_id": commit.commit_id,
            }))
        }
        let count = git_commit::Entity::find()
            .filter(
                Condition::all()
                    .add(git_commit::Column::RepoUid.eq(repo.uid))
                    .add(git_commit::Column::RefsUid.eq(branch.uid)),
            )
            .count(&self.db)
            .await?;
        Ok(json!({
            "total": count,
            "data": values
        }))
    }
}
