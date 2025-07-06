use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "issue_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub issue_id: i32,
    pub history_id: i32,
    pub action: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: i32,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
