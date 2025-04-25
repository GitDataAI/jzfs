use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::users::UsersModel;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct FollowModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub target_id: Uuid,
    pub created_at: DateTime<Utc>,
}

pub struct FollowMapper {
    pub db: sqlx::PgPool,
}

impl FollowMapper {
    pub async fn insert(&self, follow: FollowModel) -> Result<FollowModel, sqlx::Error> {
        sqlx::query_as::<_, FollowModel>(
            "INSERT INTO follow (uid, user_id, target_id, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(follow.uid)
        .bind(follow.user_id)
        .bind(follow.target_id)
        .bind(follow.created_at)
        .fetch_one(&self.db)
        .await
    }
    pub fn query(&self) -> FollowQuery {
        FollowQuery {
            db: self.db.clone(),
        }
    }
    pub fn relation(&self, follow: FollowModel) -> FollowRelation {
        FollowRelation {
            db: self.db.clone(),
            follow,
        }
    }
}

pub struct FollowQuery {
    pub db: sqlx::PgPool,
}

impl FollowQuery {
    pub async fn query_by_uid(&self, uid: Uuid) -> Result<FollowModel, sqlx::Error> {
        sqlx::query_as::<_, FollowModel>(
            "SELECT * FROM follow WHERE uid = $1",
        )
        .bind(uid)
        .fetch_one(&self.db)
        .await
    }
    pub async fn query_by_user_id(&self, user_id: Uuid) -> Result<Vec<FollowModel>, sqlx::Error> {
        sqlx::query_as::<_, FollowModel>(
            "SELECT * FROM follow WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await
    }
    pub async fn query_by_target_id(&self, target_id: Uuid) -> Result<Vec<FollowModel>, sqlx::Error> {
        sqlx::query_as::<_, FollowModel>(
            "SELECT * FROM follow WHERE target_id = $1",
        )
        .bind(target_id)
        .fetch_all(&self.db)
        .await
    }
}


pub struct FollowRelation {
    pub db: sqlx::PgPool,
    pub follow: FollowModel,
}
impl FollowRelation {
    pub async fn user(&self) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>("SELECT * FROM users WHERE uid = $1")
        .bind(self.follow.user_id)
        .fetch_one(&self.db)
        .await
        
    }
    pub async fn target(&self) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>("SELECT * FROM users WHERE uid = $1")
        .bind(self.follow.target_id)
        .fetch_one(&self.db)
        .await
    }
    pub async fn delete(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM follow WHERE uid = $1")
        .bind(self.follow.uid)
        .execute(&self.db)
        .await
        .map(|_| ())
    }
}
