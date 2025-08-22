// CREATE TABLE hybrid_recommendations (
// uid UUID PRIMARY KEY,
// user_id UUID NOT NULL,
// repo_id UUID NOT NULL,
// rank    INT    NOT NULL,
// reason  TEXT,           -- 可解释字段
// created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
// );

use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "hybrid_recommendations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repo_id: Uuid,
    pub rank: i32,
    pub reason: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
