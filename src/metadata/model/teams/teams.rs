use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "teams")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    
    pub group_id: Uuid,
    pub name: String,
    pub description: String,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    
    pub created_by: Uuid
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
