use crate::uuid_v7;
use chrono::Utc;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ssh")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub fingerprint: String,

    pub description: Option<String>,
    #[serde(skip)]
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(
        user_id: Uuid,
        name: String,
        fingerprint: String,
        description: Option<String>,
        content: String,
    ) -> Self {
        Self {
            uid: Set(uuid_v7()),
            user_id: Set(user_id),
            name: Set(name),
            fingerprint: Set(fingerprint),
            description: Set(description),
            content: Set(content),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
        }
    }
}
