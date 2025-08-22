use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_repo_tagger")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_uid: Uuid,
    pub name: String,
    pub email: String,
    pub user_uid: Option<Uuid>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
