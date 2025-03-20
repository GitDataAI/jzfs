use crate::uuid_v7;
use chrono::Utc;
use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "watch")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repo_uid: Uuid,
    pub level: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        ActiveModel {
            uid: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}

impl ActiveModel {
    pub fn new_with_user_id(user_id: Uuid, repo_uid: Uuid, level: i32) -> Self {
        ActiveModel {
            uid: Set(uuid_v7()),
            user_id: Set(user_id),
            repo_uid: Set(repo_uid),
            level: Set(level),
            created_at: Set(Utc::now().naive_utc()),
        }
    }
}
