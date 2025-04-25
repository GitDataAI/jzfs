use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct WatchModel {
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repo_uid: Uuid,
    pub level: i32,
    pub created_at: DateTime<Utc>,
}