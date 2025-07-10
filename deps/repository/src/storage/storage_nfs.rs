use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "storage_nfs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub office: bool,
    pub access_key: String,
    pub server_ip: String,
    pub server_port: i32,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
