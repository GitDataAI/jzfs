use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "email_verifications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,
    pub email: String,
    pub verification_code: String,
    pub expires_at: NaiveDateTime,
    pub verified_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub is_used: bool,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
