use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "repository")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub project: Vec<Uuid>,
    pub is_private: bool,
    pub fork: Option<Uuid>,

    pub nums_fork: i32,
    pub nums_star: i32,
    pub nums_watch: i32,
    pub nums_issue: i32,
    pub nums_release: i32,

    pub topic: Vec<String>,
    pub status: String,
    pub rtype: String,
    pub storage: Uuid,

    pub license: Option<String>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: Uuid,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
