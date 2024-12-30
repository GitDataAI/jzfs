use crate::error::{JZError, JZResult};
use crate::models::users::{follower, users};
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::prelude::Expr;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn users_follower_add(&self, uid: Uuid, follower: Uuid) -> JZResult<follower::Model> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[22] User Not Found")));
        }
        if !self.check_users_id(follower).await? {
            return Err(JZError::Other(anyhow!("[23] Follower Not Found")));
        }
        if self.check_users_follower(uid, follower).await? {
            return Err(JZError::Other(anyhow!("[24] Follower already exists")));
        }
        let txn = self.database.begin().await?;
        let result = follower::ActiveModel {
            uid: sea_orm::ActiveValue::Set(Uuid::new_v4()),
            user_id: sea_orm::ActiveValue::Set(uid),
            follower_id: sea_orm::ActiveValue::Set(follower),
            created: sea_orm::ActiveValue::Set(chrono::Local::now().timestamp()),
        }
        .insert(&txn)
        .await;
        match result {
            Ok(model) => {
                users::Entity::update_many()
                    .col_expr(
                        users::Column::Following,
                        Expr::add(Expr::col(users::Column::Following), 1),
                    )
                    .filter(users::Column::Uid.eq(uid))
                    .exec(&txn)
                    .await?;
                users::Entity::update_many()
                    .col_expr(
                        users::Column::Followers,
                        Expr::add(Expr::col(users::Column::Followers), 1),
                    )
                    .filter(users::Column::Uid.eq(follower))
                    .exec(&txn)
                    .await?;
                txn.commit().await?;
                Ok(model)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(JZError::Other(anyhow!("[25] {:?}", err)))
            }
        }
    }
    pub async fn users_follower_del(&self, uid: Uuid, follower: Uuid) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[26] User Not Found")));
        }
        if !self.check_users_id(follower).await? {
            return Err(JZError::Other(anyhow!("[27] Follower Not Found")));
        }
        if !self.check_users_follower(uid, follower).await? {
            return Err(JZError::Other(anyhow!("[28] Follower Not Found")));
        }
        let txn = self.database.begin().await?;
        let result = follower::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(follower::Column::UserId.eq(uid))
                    .add(follower::Column::FollowerId.eq(follower)),
            )
            .exec(&txn)
            .await;
        match result {
            Ok(_) => {
                users::Entity::update_many()
                    .col_expr(
                        users::Column::Following,
                        Expr::sub(Expr::col(users::Column::Following), 1),
                    )
                    .filter(users::Column::Uid.eq(uid))
                    .exec(&txn)
                    .await?;
                users::Entity::update_many()
                    .col_expr(
                        users::Column::Followers,
                        Expr::sub(Expr::col(users::Column::Followers), 1),
                    )
                    .filter(users::Column::Uid.eq(follower))
                    .exec(&txn)
                    .await?;
                txn.commit().await?;
                Ok(())
            }
            Err(err) => {
                txn.rollback().await?;
                Err(JZError::Other(anyhow!("[29] {:?}", err)))
            }
        }
    }
    pub async fn users_follower_list(&self, uid: Uuid) -> JZResult<Vec<follower::Model>> {
        let models = follower::Entity::find()
            .filter(follower::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
    pub async fn users_following_del(&self, uid: Uuid, following: Uuid) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[30] User Not Found")));
        }
        if !self.check_users_id(following).await? {
            return Err(JZError::Other(anyhow!("[31] Following Not Found")));
        }
        if !self.check_users_follower(uid, following).await? {
            return Err(JZError::Other(anyhow!("[32] Following Not Found")));
        }
        let result = follower::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(follower::Column::UserId.eq(following))
                    .add(follower::Column::FollowerId.eq(uid)),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => {
                users::Entity::update_many()
                    .col_expr(
                        users::Column::Following,
                        Expr::sub(Expr::col(users::Column::Following), 1),
                    )
                    .filter(users::Column::Uid.eq(uid))
                    .exec(&self.database)
                    .await?;
                users::Entity::update_many()
                    .col_expr(
                        users::Column::Followers,
                        Expr::sub(Expr::col(users::Column::Followers), 1),
                    )
                    .filter(users::Column::Uid.eq(following))
                    .exec(&self.database)
                    .await?;
                Ok(())
            }
            Err(err) => Err(JZError::Other(anyhow!("[33] {:?}", err))),
        }
    }
    pub async fn users_following_list(&self, uid: Uuid) -> JZResult<Vec<follower::Model>> {
        let models = follower::Entity::find()
            .filter(follower::Column::FollowerId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
}
