use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::uuid_v7;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub users_uid: Uuid,
    pub group_uid: Uuid,
    pub access: i32,
    pub join_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


impl ActiveModel {
    pub fn new(users_uid: Uuid, group_uid: Uuid, access: i32) -> Self {
        Self {
            uid: Set(uuid_v7()),
            users_uid: Set(users_uid),
            group_uid: Set(group_uid),
            access: Set(access),
            join_at: Set(Utc::now().naive_utc())
        }
    }
}