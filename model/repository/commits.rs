use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "commit")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub uid: Uuid,
    pub id: String,
    pub branch_uid: Uuid,
    pub repo_uid: Uuid,
    pub branch_name: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub status: String,
    pub runner: Vec<String>,
    pub time: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}