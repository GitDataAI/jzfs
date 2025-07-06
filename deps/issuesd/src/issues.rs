use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "issues")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub issue_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub author_id: i32,
    pub assignee_id: Option<i32>,
    pub state: String,
    pub priority_label_id: Option<i32>, 
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
