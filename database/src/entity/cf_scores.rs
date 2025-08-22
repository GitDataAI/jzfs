// CREATE TABLE cf_scores (
// cf_uid   UUID PRIMARY KEY,
// user_id UUID NOT NULL,
// repo_id UUID NOT NULL,
// score   REAL   NOT NULL,
// updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
// created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
// )

use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cf_scores")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cf_uid: Uuid,
    pub user_id: Uuid,
    pub repo_id: Uuid,
    pub score: f32,
    pub updated_at: DateTime,
    pub created_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
