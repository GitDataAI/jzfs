use lib_entity::ActiveModelTrait;
use lib_entity::ActiveValue::Set;
use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::IntoActiveModel;
use lib_entity::QueryFilter;
use lib_entity::TransactionTrait;
use lib_entity::prelude::Uuid;
use lib_entity::sqlx::types::chrono::Utc;
use lib_entity::users::follower;
use lib_entity::users::users;
use serde::Deserialize;
use serde::Serialize;

use crate::server::AppUserState;

#[derive(Deserialize, Serialize)]
pub struct FollowParma {
    pub uid : Uuid,
    pub follow_uid : Uuid,
}

impl AppUserState {
    pub async fn follow(&self, uid : Uuid, follow_uid : Uuid) -> anyhow::Result<()> {
        let mut tx = self.read.begin().await?;

        let user = users::Entity::find_by_id(uid)
            .one(&mut tx)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        let follow_user = users::Entity::find_by_id(follow_uid)
            .one(&mut tx)
            .await?
            .ok_or_else(|| anyhow::anyhow!("follow user not found"))?;

        let follow = follower::Entity::find()
            .filter(follower::Column::UserId.eq(user.uid))
            .filter(follower::Column::FollowerId.eq(follow_user.uid))
            .one(&mut tx)
            .await?;
        if follow.is_some() {
            return Err(anyhow::anyhow!("already follow"));
        }

        follower::ActiveModel {
            uid : Set(Uuid::new_v4()),
            user_id : Set(user.uid),
            follower_id : Set(follow_user.uid),
            created : Set(Utc::now().timestamp()),
        }
        .insert(&mut tx)
        .await?;

        let mut user_active = user.clone().into_active_model();
        let mut target_active = follow_user.clone().into_active_model();

        user_active.followers = Set(user.followers + 1);
        target_active.following = Set(follow_user.following + 1);

        user_active.update(&mut tx).await?;
        target_active.update(&mut tx).await?;

        tx.commit().await?;
        Ok(())
    }
    pub async fn unfollow(&self, uid : Uuid, follow_uid : Uuid) -> anyhow::Result<()> {
        let mut tx = self.read.begin().await?;

        let user = users::Entity::find_by_id(uid)
            .one(&mut tx)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let follow_user = users::Entity::find_by_id(follow_uid)
            .one(&mut tx)
            .await?
            .ok_or_else(|| anyhow::anyhow!("follow user not found"))?;

        let follow = follower::Entity::find()
            .filter(follower::Column::UserId.eq(user.uid))
            .filter(follower::Column::FollowerId.eq(follow_user.uid))
            .one(&mut tx)
            .await?;

        if follow.is_none() {
            return Err(anyhow::anyhow!("not follow"));
        }
        let follow = follow.unwrap();
        follow.into_active_model().delete(&mut tx).await?;

        let mut user_active = user.clone().into_active_model();
        let mut target_active = follow_user.clone().into_active_model();

        user_active.followers = Set(user.followers - 1);
        target_active.following = Set(follow_user.following - 1);

        user_active.update(&mut tx).await?;
        target_active.update(&mut tx).await?;

        tx.commit().await?;
        Ok(())
    }
    pub async fn follow_list(&self, uid : Uuid) -> anyhow::Result<Vec<users::Model>> {
        let mut tx = self.read.begin().await?;

        let user = users::Entity::find_by_id(uid)
            .one(&mut tx)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let follow_list = follower::Entity::find()
            .filter(follower::Column::UserId.eq(user.uid))
            .all(&mut tx)
            .await?;

        let mut uids = vec![];
        for follow in follow_list {
            uids.push(follow.follower_id);
        }

        let follow_list = users::Entity::find()
            .filter(users::Column::Uid.is_in(uids))
            .all(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(follow_list)
    }
}
