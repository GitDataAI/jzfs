use crate::AppCore;
use database::entity::user_follow;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::sqlx::types::uuid;
use sea_orm::{ColumnTrait, PaginatorTrait};

impl AppCore {
    pub async fn users_follow_is(&self, user_uid: uuid::Uuid, follow_uid: uuid::Uuid) -> bool {
        if let Ok(count) = user_follow::Entity::find()
            .filter(user_follow::Column::UserUid.eq(user_uid))
            .filter(user_follow::Column::FollowUid.eq(follow_uid))
            .count(&self.db)
            .await
        {
            count > 0
        } else {
            false
        }
    }
}
