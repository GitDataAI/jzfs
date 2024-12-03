use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "groups")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub description: String,
    pub avatar: Option<String>,
    
    pub website: Vec<String>,
    pub location: String,
    pub unit: Option<String>,
    
    pub contact: String,
    pub owner: Uuid,
    
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}



#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}