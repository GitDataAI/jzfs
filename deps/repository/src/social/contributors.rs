use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "contributors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    #[sea_orm(auto_increment = true)]
    pub id: i32,
    pub user_id: Option<Uuid>,
    pub repo_id: Uuid,
    pub email: String,
    pub name: String,
}



impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}