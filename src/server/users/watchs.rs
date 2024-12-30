use crate::error::{JZError, JZResult};
use crate::models::repos::repos;
use crate::models::users::watchs;
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::{prelude::*, *};
use uuid::Uuid;

impl MetaData {
    pub async fn users_watchs_add(&self, uid: Uuid, repo_id: Uuid, level: i32) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[57] User Not Found")));
        }
        if !self.check_repo_id(repo_id).await? {
            return Err(JZError::Other(anyhow!("[58] Repo Not Found")));
        }
        let result = watchs::Entity::insert(watchs::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(uid),
            repo_id: Set(repo_id),
            level: Set(level),
            created_at: Default::default(),
        })
        .exec(&self.database)
        .await;
        match result {
            Ok(_) => {
                repos::Entity::update_many()
                    .filter(repos::Column::Uid.eq(repo_id))
                    .col_expr(
                        repos::Column::NumsWatcher,
                        Expr::add(Expr::col(repos::Column::NumsWatcher), 1),
                    )
                    .exec(&self.database)
                    .await?;
                Ok(())
            }
            Err(err) => Err(JZError::Other(anyhow!("[59] {:?}", err))),
        }
    }
    pub async fn users_watchs_remove(&self, uid: Uuid, repo_id: Uuid) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[60] User Not Found")));
        }
        if !self.check_repo_id(repo_id).await? {
            return Err(JZError::Other(anyhow!("[61] Repo Not Found")));
        }
        let result = watchs::Entity::delete_many()
            .filter(watchs::Column::UserId.eq(uid))
            .filter(watchs::Column::RepoId.eq(repo_id))
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => {
                repos::Entity::update_many()
                    .filter(repos::Column::Uid.eq(repo_id))
                    .col_expr(
                        repos::Column::NumsWatcher,
                        Expr::sub(Expr::col(repos::Column::NumsWatcher), 1),
                    )
                    .exec(&self.database)
                    .await?;
                Ok(())
            }
            Err(err) => Err(JZError::Other(anyhow!("[62] {:?}", err))),
        }
    }
    pub async fn users_watchs_update(&self, uid: Uuid, repo_id: Uuid, level: i32) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[63] User Not Found")));
        }
        if !self.check_repo_id(repo_id).await? {
            return Err(JZError::Other(anyhow!("[64] Repo Not Found")));
        }
        let result = watchs::Entity::update_many()
            .filter(watchs::Column::UserId.eq(uid))
            .filter(watchs::Column::RepoId.eq(repo_id))
            .col_expr(watchs::Column::Level, Expr::value(level))
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[65] {:?}", err))),
        }
    }
    pub async fn users_watchs_list(&self, uid: Uuid) -> JZResult<Vec<watchs::Model>> {
        let watchs = watchs::Entity::find()
            .filter(watchs::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(watchs)
    }
}
