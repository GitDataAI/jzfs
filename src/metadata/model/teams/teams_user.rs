use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "teams_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub join_at: OffsetDateTime,
    pub access: i32 // 0 read / 1 write / 2 admin / 3 owner
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
