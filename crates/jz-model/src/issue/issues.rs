use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "issues")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub title: String,
    pub repo_uid: Uuid,
    pub created_by: Uuid,

    pub body: String,
    pub state: String,

    pub tags: Vec<Uuid>,

    pub closed_at: Option<DateTime>,
    pub closed_by: Option<Uuid>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
