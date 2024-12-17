use sea_orm::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel, ToSchema)]
#[sea_orm(table_name = "groups_labels")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub label: String,
    pub color: String,
    pub group_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}