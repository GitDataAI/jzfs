use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,

    pub username: String,
    pub email: String,

    pub description: Option<String>,
    pub website: Option<String>,
    pub avatar: Option<String>,

    pub timezone: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
    pub location: Option<String>,
    pub topic: Vec<String>,

    pub setting: Vec<String>,

    pub active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    
    pub created_by: Uuid,
    pub owner_org: Option<String>,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

