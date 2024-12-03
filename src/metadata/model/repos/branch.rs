use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "branch")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: String,
    pub branch: String,

    pub protect: bool,
    pub visible: bool,

    pub head: Uuid, // 最近一次commit的uid

    pub created_at: i64,
    pub updated_at: i64,

    pub created_by: Uuid,

}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
