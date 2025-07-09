use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "issues")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    #[sea_orm(auto_increase = true)]
    pub issue_id: i32,
    pub repo_uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_uid: Uuid,
    pub assignee_uid: Option<Vec<Uuid>>,
    pub state: String,
    pub priority_label_uid: Option<Uuid>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
