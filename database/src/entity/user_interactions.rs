// CREATE TYPE action AS ENUM ('clone','star','fork','commit','pr','view');
// CREATE TABLE user_interactions (
// id          UUID PRIMARY KEY,
// user_id     UUID NOT NULL,
// repo_id     UUID NOT NULL,
// act         action NOT NULL,
// created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
// weight      REAL    NOT NULL DEFAULT 1.0   -- 用于加权 CF
// );
// CREATE INDEX ix_ui_user_time ON user_interactions(user_id, created_at DESC);
// CREATE INDEX ix_ui_repo_time ON user_interactions(repo_id, created_at DESC);

use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_interactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repo_id: Uuid,
    pub act: Interaction,
    pub created_at: DateTime,
    pub weight: f32,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "interaction_type")]
pub enum Interaction {
    #[sea_orm(string_value = "clone")]
    Clone,
    #[sea_orm(string_value = "star")]
    Star,
    #[sea_orm(string_value = "fork")]
    Fork,
    #[sea_orm(string_value = "commit")]
    Commit,
    #[sea_orm(string_value = "pr")]
    Pr,
    #[sea_orm(string_value = "view")]
    View,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
