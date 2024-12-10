use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "users_email")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub is_public: bool,
    pub verified: bool,
    pub bind_at: OffsetDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}