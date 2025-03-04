use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "data_product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    
    pub hash: String,
    pub size: i64,
    pub owner: Uuid,
    pub repository_uid: Uuid,
    
    pub r#type: String,
    pub license: String,
    
    pub price: Option<i64>,
    
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}