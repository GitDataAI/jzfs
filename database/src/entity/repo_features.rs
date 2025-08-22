use sea_orm::entity::prelude::*;
use sea_orm::prelude::PgVector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "repo_features")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub repo_uid: Uuid,
    pub vector: PgVector,
    pub meta: Json,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
