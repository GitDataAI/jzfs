use chrono::{Duration, Utc};
use uuid::Uuid;

use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Option<Uuid>,
    pub session_token: String,
    pub ip_address: Option<String>,
    pub value: String,
    pub user_agent: Option<String>,
    pub device_info: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub last_used_at: chrono::NaiveDateTime,
    pub is_active: bool,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModel {
    pub fn new() -> Self {
        Self {
            uid: Set(Uuid::now_v7()),
            user_id: Set(None),
            session_token: Set(String::new()),
            ip_address: Set(None),
            value: Set(String::new()),
            user_agent: Set(None),
            device_info: Set(None),
            created_at: Set(Utc::now().naive_utc()),
            expires_at: Set(Utc::now().naive_utc() + Duration::days(7)),
            last_used_at: Set(Utc::now().naive_utc()),
            is_active: Set(true),
        }
    }
}
