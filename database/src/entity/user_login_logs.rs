use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_login_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,
    pub login_method: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub location_info: Option<Json>,
    pub success: bool,
    pub failure_reason: Option<String>,
    pub created_at: NaiveDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
