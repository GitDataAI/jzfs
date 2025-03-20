use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "invite")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub org_uid: Uuid,
    pub team_uid: Option<Uuid>,
    pub user_uid: Uuid,
    pub email: Option<String>,
    pub access: i32,
    pub status: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
