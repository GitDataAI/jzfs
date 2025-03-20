use crate::uuid_v7;
use chrono::Utc;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "stars")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(user_id: Uuid, repository_id: Uuid) -> Self {
        Self {
            uid: Set(uuid_v7()),
            user_id: Set(user_id),
            repository_id: Set(repository_id),
            created_at: Set(Utc::now().naive_utc()),
        }
    }
}
