use std::io;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use uuid::Uuid;
use crate::app::services::AppState;
use crate::app::services::statistics::repo::{STAR, WATCH};
use crate::model::origin::organization;
use crate::model::repository::repository;
use crate::model::users::{star, users, watch};

impl AppState {
    pub async fn repo_info(
        &self,
        owner: String, 
        repo: String
    )
    -> io::Result<repository::Model>
    {
        let owner_uid =if let Some(x) = users::Entity::find()
            .filter(users::Column::Username.eq(owner.clone()))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
        {
            x.uid
        }else {
            organization::Entity::find()
                .filter(organization::Column::Username.eq(owner))
                .one(&self.read)
                .await
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
                .ok_or(io::Error::new(io::ErrorKind::Other, "owner not found"))?
                .uid
        };
        repository::Entity::find()
            .filter(repository::Column::OwnerId.eq(owner_uid))
            .filter(repository::Column::Name.eq(repo))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))
    }
    pub async fn repo_get_by_uid(&self, uid: Uuid) -> io::Result<repository::Model> {
        repository::Entity::find_by_id(uid)
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))
    }
    pub async fn repo_star(&self, users_uid: Uuid, repos_uid: Uuid) -> io::Result<()> {
        let repo = repository::Entity::find_by_id(repos_uid)
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))?;
        let user = users::Entity::find_by_id(users_uid)
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "user not found"))?;
        let star = star::Entity::find()
            .filter(star::Column::UserId.eq(user.uid))
            .filter(star::Column::RepositoryId.eq(repo.uid))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        let txn = self.write.begin().await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        match star {
            Some(x) => {
                star::Entity::delete_many()
                    .filter(star::Column::Uid.eq(x.uid))
                    .filter(star::Column::UserId.eq(user.uid))
                    .filter(star::Column::RepositoryId.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsStar, Expr::col(repository::Column::NumsStar).sub(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                txn.commit().await.map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                self.statistics_repo(repos_uid, STAR.to_string()).await.ok();
                Ok(())
            },
            None => {
                star::ActiveModel {
                    uid: Set(Uuid::new_v4()),
                    user_id: Set(user.uid),
                    repository_id: Set(repo.uid),
                    created_at: Set(Utc::now().naive_utc()),
                    
                }.insert(&txn).await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsStar, Expr::col(repository::Column::NumsStar).add(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                self.statistics_repo(repos_uid, STAR.to_string()).await.ok();
                txn.commit().await.map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                Ok(())
            }
        }
    }
    pub async fn repo_watch(&self, users_uid: Uuid, repos_uid: Uuid, level: i32) -> io::Result<()> {
        let repo = repository::Entity::find_by_id(repos_uid)
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))?;
        let user = users::Entity::find_by_id(users_uid)
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "user not found"))?;
        let watch = watch::Entity::find()
            .filter(watch::Column::UserId.eq(user.uid))
            .filter(watch::Column::RepositoryId.eq(repo.uid))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        let txn = self.write.begin().await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        match watch {
            Some(x) => {
                watch::Entity::delete_many()
                    .filter(watch::Column::Uid.eq(x.uid))
                    .filter(watch::Column::UserId.eq(user.uid))
                    .filter(watch::Column::RepositoryId.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsWatch, Expr::col(repository::Column::NumsWatch).sub(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                txn.commit().await.map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                self.statistics_repo(repos_uid, WATCH.to_string()).await.ok();
                Ok(())
            },
            None => {
                watch::ActiveModel {
                    uid: Set(Uuid::new_v4()),
                    user_id: Set(user.uid),
                    repository_id: Set(repo.uid),
                    created_at: Set(Utc::now().naive_utc()),
                    level: Set(level),
                }.insert(&txn).await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsWatch, Expr::col(repository::Column::NumsWatch).add(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                self.statistics_repo(repos_uid, WATCH.to_string()).await.ok();
                txn.commit().await.map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                Ok(())
            }
        }
    }
}