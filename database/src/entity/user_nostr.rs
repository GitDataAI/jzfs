use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_nostr")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,

    pub relay: String,
    pub pubkey: String,
    pub seckey: String,
    pub pin_code: String,
    pub is_active: bool,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
