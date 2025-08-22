use crate::{AppCore, Paginator};
use database::git_repo_stats;
use database::user_interactions::Interaction;
use database::user_star_repo;
use sea_orm::PaginatorTrait;
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{
    ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, Set, TransactionTrait,
};
use serde_json::json;
use session::Session;

impl AppCore {
    pub async fn repos_stats(
        &self,
        namespace: &str,
        repo_name: &str,
    ) -> Option<git_repo_stats::Model> {
        let repo = database::git_repo::Entity::find()
            .filter(database::git_repo::Column::Namespace.eq(namespace))
            .filter(database::git_repo::Column::RepoName.eq(repo_name))
            .one(&self.db)
            .await
            .ok()
            .flatten()?;
        git_repo_stats::Entity::find()
            .filter(git_repo_stats::Column::RepoUid.eq(repo.uid))
            .one(&self.db)
            .await
            .ok()
            .flatten()
    }

    pub async fn user_starred_repo(
        &self,
        namespace: &str,
        repo_name: &str,
        session: Session,
    ) -> bool {
        let repo = match database::git_repo::Entity::find()
            .filter(database::git_repo::Column::Namespace.eq(namespace))
            .filter(database::git_repo::Column::RepoName.eq(repo_name))
            .one(&self.db)
            .await
            .ok()
            .flatten()
        {
            Some(r) => r,
            None => return false,
        };
        let user = match self.user_context(session).await {
            Ok(u) => u,
            Err(_) => return false,
        };
        self.inner_add_interaction(user.user_uid, repo.uid, Interaction::Star)
            .await
            .ok();
        user_star_repo::Entity::find()
            .filter(user_star_repo::Column::UserId.eq(user.user_uid))
            .filter(user_star_repo::Column::RepoId.eq(repo.uid))
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .is_some()
    }

    pub async fn star_repo(
        &self,
        namespace: &str,
        repo_name: &str,
        session: Session,
    ) -> Result<(), sea_orm::DbErr> {
        let txn = self.db.begin().await?;
        let repo = database::git_repo::Entity::find()
            .filter(database::git_repo::Column::Namespace.eq(namespace))
            .filter(database::git_repo::Column::RepoName.eq(repo_name))
            .one(&txn)
            .await
            .ok()
            .flatten()
            .ok_or(sea_orm::DbErr::Custom("Repo not found".into()))?;
        let user = self
            .user_context(session)
            .await
            .map_err(|_| sea_orm::DbErr::Custom("Not login".into()))?;
        let now = Utc::now();
        let uid = Uuid::now_v7();
        let active = user_star_repo::ActiveModel {
            uid: Set(uid),
            user_id: Set(user.user_uid),
            repo_id: Set(repo.uid),
            created_at: Set(now.into()),
        };
        user_star_repo::Entity::insert(active).exec(&txn).await?;
        if let Some(stats) = git_repo_stats::Entity::find()
            .filter(git_repo_stats::Column::RepoUid.eq(repo.uid))
            .one(&txn)
            .await?
        {
            let mut active = stats.clone().into_active_model();
            active.stars = Set(stats.stars + 1);
            git_repo_stats::Entity::update(active).exec(&txn).await?;
        }
        txn.commit().await?;
        Ok(())
    }
    pub async fn unstar_repo(
        &self,
        namespace: &str,
        repo_name: &str,
        session: Session,
    ) -> Result<(), sea_orm::DbErr> {
        let txn = self.db.begin().await?;
        let repo = database::git_repo::Entity::find()
            .filter(database::git_repo::Column::Namespace.eq(namespace))
            .filter(database::git_repo::Column::RepoName.eq(repo_name))
            .one(&txn)
            .await
            .ok()
            .flatten()
            .ok_or(sea_orm::DbErr::Custom("Repo not found".into()))?;
        let user = self
            .user_context(session)
            .await
            .map_err(|_| sea_orm::DbErr::Custom("Not login".into()))?;
        user_star_repo::Entity::delete_many()
            .filter(user_star_repo::Column::UserId.eq(user.user_uid))
            .filter(user_star_repo::Column::RepoId.eq(repo.uid))
            .exec(&txn)
            .await?;
        if let Some(stats) = git_repo_stats::Entity::find()
            .filter(git_repo_stats::Column::RepoUid.eq(repo.uid))
            .one(&txn)
            .await?
        {
            let mut active = stats.clone().into_active_model();
            active.stars = Set(stats.stars - 1);
            git_repo_stats::Entity::update(active).exec(&txn).await?;
        }
        txn.commit().await?;
        Ok(())
    }
    pub async fn users_star_repos(
        &self,
        username: &str,
        paginator: Paginator,
    ) -> anyhow::Result<serde_json::Value> {
        let user = database::users::Entity::find()
            .filter(database::users::Column::Username.eq(username))
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .ok_or(anyhow::anyhow!("User not found"))?;
        let repos = user_star_repo::Entity::find()
            .filter(user_star_repo::Column::UserId.eq(user.uid))
            .offset(paginator.page * paginator.page_size)
            .limit(paginator.page_size)
            .all(&self.db)
            .await?;
        let mut result = Vec::new();
        for repo in repos {
            let git_repo = database::git_repo::Entity::find()
                .filter(database::git_repo::Column::Uid.eq(repo.repo_id))
                .one(&self.db)
                .await?;
            if let Some(git_repo) = git_repo {
                result.push(git_repo);
            }
        }
        let total = user_star_repo::Entity::find()
            .filter(user_star_repo::Column::UserId.eq(user.uid))
            .count(&self.db)
            .await?;
        Ok(json!({
            "total": total,
            "items": result
        }))
    }
}
