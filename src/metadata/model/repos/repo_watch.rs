use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "repo_watch")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repo_id: Uuid,
    pub mode: i64,
    
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}