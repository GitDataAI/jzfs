use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "git_commit")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub sha: String,
    pub branch_uid: Uuid,
    pub repo_uid: Uuid,
    pub branch_name: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub commiter_name: String,
    pub commiter_email: String,
    pub status: i32,
    pub runner: Vec<Uuid>,
    pub time: String,
    pub created_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
