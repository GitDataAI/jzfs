use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub group_uid: Uuid,
    pub secret: bool,
    pub access: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
