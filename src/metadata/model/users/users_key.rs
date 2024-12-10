use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "users_key")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub pubkey: Vec<u8>,
    
    pub hash: String,
    pub expire_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub last_use: OffsetDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}