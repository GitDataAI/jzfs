use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "repo_branch")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch: String,

    pub protect: bool,
    pub visible: bool,

    pub head: Option<Uuid>, // 最近一次commit的uid

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,

    pub created_by: Uuid,

}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
