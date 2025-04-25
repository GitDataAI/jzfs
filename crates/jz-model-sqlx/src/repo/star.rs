use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct StarModel {
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub created_at: DateTime<Utc>,
}