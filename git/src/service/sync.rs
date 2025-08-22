use crate::GitContext;
use crate::object::commit::CommitPaginator;
use crate::service::GitServer;
use anyhow::anyhow;
use database::entity::{
    git_commit, git_refs, git_repo, git_tag, user_repo_active, user_repo_tagger, users,
};
use error::AppError;
use redis::AsyncCommands;
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set,
    TransactionTrait, TryIntoModel,
};
use std::collections::HashSet;

impl GitServer {
    pub async fn sync_repo(&self, repo_uid: Uuid) -> Result<(), AppError> {
        let mut redis_client = self
            .redis
            .get()
            .await
            .map_err(|_| AppError::from(anyhow!("Redis error")))?;
        redis_client
            .set_ex::<String, i32, ()>(
                format!("git:repo:{}:sync", repo_uid.to_string()),
                1,
                60 * 10,
            )
            .await?;
        let repo = git_repo::Entity::find_by_id(repo_uid)
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Repo not found")))?;
        let git = GitContext::try_from((repo, self.config.git.clone()))?;
        let refs = git.refs_list()?;
        let txn = self.db.begin().await?;
        let db_refs = git_refs::Entity::find()
            .filter(Condition::all().add(git_refs::Column::RepoUid.eq(repo_uid)))
            .all(&txn)
            .await?;

        for ref_item in refs {
            let ref_item =
                if let Some(db_ref) = db_refs.iter().find(|x| x.ref_name == ref_item.name) {
                    if db_ref.ref_git_id != ref_item.hash {
                        let mut ref_active = db_ref.clone().into_active_model();
                        ref_active.ref_git_id = Set(ref_item.hash);
                        ref_active.updated_at = Set(Utc::now().naive_utc());
                        ref_active.update(&txn).await?
                    } else {
                        continue;
                    }
                } else {
                    let ref_active = git_refs::ActiveModel {
                        uid: Set(Uuid::now_v7()),
                        repo_uid: Set(repo_uid.clone()),
                        ref_name: Set(ref_item.name),
                        ref_git_id: Set(ref_item.hash),
                        default_branch: Set(ref_item.is_head),
                        created_at: Set(Utc::now().naive_utc()),
                        updated_at: Set(Utc::now().naive_utc()),
                    };
                    ref_active.insert(&txn).await?
                };
            let commit = git.commit_list(CommitPaginator {
                start_oid: None,
                end_oid: None,
                refs: Some(ref_item.ref_name),
            })?;
            let commits_hash = commit
                .clone()
                .iter()
                .map(|x| x.commit_oid.clone())
                .collect::<Vec<_>>();
            let mut need_commit_hash = HashSet::new();
            let db_hash = git_commit::Entity::find()
                .filter(
                    Condition::all()
                        .add(git_commit::Column::RepoUid.eq(repo_uid))
                        .add(git_commit::Column::CommitId.is_in(commits_hash)),
                )
                .all(&txn)
                .await?
                .iter()
                .map(|x| x.commit_id.clone())
                .collect::<Vec<_>>();
            for commit_item in commit {
                if !db_hash.contains(&commit_item.commit_oid) {
                    need_commit_hash.insert(commit_item);
                }
            }
            for commit_item in need_commit_hash {
                let commiter = commit_item.committer;
                let authors = commit_item.author;
                let mut commit_active = git_commit::ActiveModel {
                    uid: Set(Uuid::now_v7()),
                    repo_uid: Set(repo_uid.clone()),
                    refs_uid: Set(ref_item.uid),
                    commit_id: Set(commit_item.commit_oid),
                    tree: Set(commit_item.tree_oid),
                    parents_id: Set(serde_json::to_value(commit_item.parents)?),
                    author: Set(None),
                    committer: Set(None),
                    content: Set(commit_item.message),
                    time: Set(commit_item.time),
                    offset: Set(commit_item.offset_date),
                };
                let commit = commit_active.clone().try_into_model()?;
                let (author_uid, commiter_uid) = if commiter == authors {
                    let user_uid = users::Entity::find()
                        .filter(
                            Condition::all().add(users::Column::Email.eq(authors.email.clone())),
                        )
                        .one(&txn)
                        .await?
                        .map(|x| x.uid);
                    let authors = user_repo_active::ActiveModel {
                        uid: Set(Uuid::now_v7()),
                        name: Set(authors.name.clone()),
                        email: Set(authors.email),
                        user_uid: Set(user_uid),
                        commit: Set(commit.uid.clone()),
                        repo_uid: Set(repo_uid.clone()),
                        time: Set(commit.time),
                        offset: Set(commit.offset),
                    };
                    let au = authors.insert(&txn).await?;
                    (au.uid, au.uid)
                } else {
                    let user_uid = users::Entity::find()
                        .filter(
                            Condition::all().add(users::Column::Email.eq(authors.email.clone())),
                        )
                        .one(&txn)
                        .await?
                        .map(|x| x.uid);
                    let authors = user_repo_active::ActiveModel {
                        uid: Set(Uuid::now_v7()),
                        name: Set(authors.name.clone()),
                        email: Set(authors.email),
                        user_uid: Set(user_uid),
                        commit: Set(commit.uid.clone()),
                        repo_uid: Set(repo_uid.clone()),
                        time: Set(commit.time),
                        offset: Set(commit.offset),
                    };
                    let au = authors.insert(&txn).await?;
                    let user_uid = users::Entity::find()
                        .filter(
                            Condition::all().add(users::Column::Email.eq(commiter.email.clone())),
                        )
                        .one(&txn)
                        .await?
                        .map(|x| x.uid);
                    let commiter = user_repo_active::ActiveModel {
                        uid: Set(Uuid::now_v7()),
                        name: Set(commiter.name),
                        email: Set(commiter.email),
                        user_uid: Set(user_uid),
                        commit: Set(commit.uid.clone()),
                        repo_uid: Set(repo_uid.clone()),
                        time: Set(commit.time),
                        offset: Set(commit.offset),
                    };
                    let cm = commiter.insert(&txn).await?;
                    (au.uid, cm.uid)
                };
                commit_active.committer = Set(Some(commiter_uid));
                commit_active.author = Set(Some(author_uid));
                commit_active.insert(&txn).await?;
            }
        }
        let tags = git.tag_list()?;
        for tag in tags {
            if git_tag::Entity::find()
                .filter(
                    Condition::all()
                        .add(git_tag::Column::RepoUid.eq(repo_uid.clone()))
                        .add(git_tag::Column::TagName.eq(tag.tag_name.clone()))
                        .add(git_tag::Column::TagId.eq(tag.tag_id.clone())),
                )
                .one(&txn)
                .await?
                .is_some()
            {
                continue;
            } else {
                let tagger = tag.tag_tagger;
                let tagger = if let Some(tagger) = tagger {
                    let users = users::Entity::find()
                        .filter(Condition::all().add(users::Column::Email.eq(tagger.email.clone())))
                        .one(&txn)
                        .await?
                        .map(|x| x.uid);
                    if let Some(users) = users {
                        let tagger = user_repo_tagger::Entity::find()
                            .filter(
                                Condition::all()
                                    .add(user_repo_tagger::Column::UserUid.eq(users.clone()))
                                    .add(user_repo_tagger::Column::RepoUid.eq(repo_uid.clone())),
                            )
                            .one(&txn)
                            .await?;
                        if let Some(tagger) = tagger {
                            Some(tagger)
                        } else {
                            let tagger = user_repo_tagger::ActiveModel {
                                uid: Set(Uuid::now_v7()),
                                user_uid: Set(Option::from(users)),
                                repo_uid: Set(repo_uid.clone()),
                                name: Set(tag.tag_name.clone()),
                                email: Set(tag.tag_msg.clone()),
                            };
                            let model = tagger.insert(&txn).await?;
                            Some(model)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
                let tag = git_tag::ActiveModel {
                    uid: Set(Uuid::now_v7()),
                    repo_uid: Set(repo_uid.clone()),
                    tag_id: Set(tag.tag_id),
                    tag_name: Set(tag.tag_name),
                    tagger: Set(tagger.map(|x| x.uid)),
                    message: Set(tag.tag_msg),
                };
                tag.insert(&txn).await?;
            }
        }
        txn.commit().await?;
        redis_client
            .del::<String, ()>(format!("git:repo:{}:sync", repo_uid))
            .await?;
        Ok(())
    }
}
