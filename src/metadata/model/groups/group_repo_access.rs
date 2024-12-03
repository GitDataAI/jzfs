use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "group_repo_access")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub repo_id: Uuid,
    pub group_id: Uuid,
    pub team_id: Uuid,
    pub access: i32, // 0 read / 1 write / 2 admin / 3 owner
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}