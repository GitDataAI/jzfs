use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub label_uid: Uuid,
    pub repo_uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
