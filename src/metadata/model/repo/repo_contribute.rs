use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "repo_contribute")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: String,
    pub user_id: Uuid,
    pub repo_id: Uuid,
    pub contribute: String,
    
    pub first_at: OffsetDateTime,
    pub last_at: OffsetDateTime,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}