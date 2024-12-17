use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "groups_invite")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub email: String,

    pub status: i32, // 0 wait / 1 ok / -1 no

    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
    pub invited_by: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
