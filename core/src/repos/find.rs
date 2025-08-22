use crate::{AppCore, Paginator};
use anyhow::anyhow;
use database::entity::git_repo;
use database::{git_repo_stats, user_repo, users};
use error::AppError;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::{Condition, PaginatorTrait};
use serde_json::json;
use session::Session;

impl AppCore {
    pub async fn repo_find(&self, owner: &str, name: &str) -> Result<git_repo::Model, AppError> {
        let repo = git_repo::Entity::find()
            .filter(git_repo::Column::RepoName.eq(name))
            .filter(git_repo::Column::Namespace.eq(owner))
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Repo not found")))?;
        Ok(repo)
    }
    pub async fn repo_find_by_owner(
        &self,
        owner: &str,
        session: Session,
        paginator: Paginator,
    ) -> Result<serde_json::Value, AppError> {
        let user = self.user_context_current(session).await.ok();
        let mut condition = Condition::all();
        condition = condition.add(git_repo::Column::Namespace.eq(owner));
        if let Some(user) = user {
            if user.username != owner {
                condition = condition.add(git_repo::Column::IsPrivate.eq(false));
            }
        } else {
            condition = condition.add(git_repo::Column::IsPrivate.eq(false));
        }
        let repos = git_repo::Entity::find()
            .filter(condition)
            .order_by_desc(git_repo::Column::CreatedAt)
            .limit(paginator.page_size)
            .offset(paginator.page_size * paginator.page)
            .all(&self.db)
            .await?;
        let total = git_repo::Entity::find()
            .filter(git_repo::Column::Namespace.eq(owner))
            .count(&self.db)
            .await?;
        let mut result = Vec::new();
        for repo in repos {
            match user_repo::Entity::find()
                .filter(Condition::all().add(user_repo::Column::RepoUid.eq(repo.uid)))
                .one(&self.db)
                .await
            {
                Ok(Some(owner)) => {
                    match users::Entity::find_by_id(owner.user_uid)
                        .one(&self.db)
                        .await
                    {
                        Ok(Some(owner)) => {
                            let state = git_repo_stats::Entity::find()
                                .filter(git_repo_stats::Column::RepoUid.eq(repo.uid))
                                .one(&self.db)
                                .await?;
                            let json = json!({
                                "repo": repo,
                                "owner": {
                                    "uid": owner.uid,
                                    "username": owner.username,
                                    "avatar_url": owner.avatar_url,
                                },
                                "state": state,
                            });
                            result.push(json);
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
        Ok(json!({
            "total": total,
            "items": result,
            "page": paginator.page,
            "page_size": paginator.page_size
        }))
    }
}
