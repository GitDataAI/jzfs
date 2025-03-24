use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::uuid_v7;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub color: String,
    pub description: Option<String>,
    pub repo_uid: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


impl ActiveModel {
    pub fn new(
        color: String,
        description: Option<String>,
        repo_uid: Uuid
    ) -> Self {
        ActiveModel {
            uid: Set(Uuid::new_v4()),
            color: Set(color),
            description: Set(description),
            repo_uid: Set(repo_uid),
            created_by: Set(uuid_v7()),
            created_at: Set(Utc::now().naive_utc()),
        }
    }
}