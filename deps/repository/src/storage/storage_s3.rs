use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "storage_s3")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub office: bool,
    pub server_ip: String,
    pub server_port: i32,
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
    pub endpoint: String,
    pub region: String,
    pub path: String,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
