use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "security")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,

    pub title: String,
    pub description: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub device: Option<String>,
    pub location: Option<String>,

    pub action: String,
    pub actor: String,
    pub actor_uid: Uuid,

    pub user: String,
    pub user_uid: Uuid,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
